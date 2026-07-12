use url::Url;

/// Whether `host` is a Google account domain, including locale ccTLDs. Sign-in
/// propagates the session across domains via a SetSID/SetOSID redirect chain that
/// bounces through `accounts.google.<tld>` (e.g. `accounts.google.pl`), so an
/// exact `accounts.google.com` match is not enough.
fn is_google_account_host(host: &str) -> bool {
    matches!(
        host,
        "accounts.youtube.com" | "myaccount.google.com" | "workspace.google.com"
    ) || host.starts_with("accounts.google.")
}

pub fn is_whitelisted(url: &Url) -> bool {
    if !matches!(url.scheme(), "http" | "https") {
        return false;
    }
    let Some(host) = url.host_str() else {
        return false;
    };

    // Authentication and account flows (sign-in, password, 2FA, cross-domain SID
    // cookie setters). Kept in the webview so login completes in-app.
    if is_google_account_host(host) {
        return true;
    }

    match host {
        // Google Chat / Spaces - block direct attachment downloads (those go external).
        "chat.google.com" => !url.path().starts_with("/u/0/api/get_attachment_url"),

        // Gmail host - the /chat/* subtree plus the /accounts session setters
        // (SetOSID) the login flow redirects through. Everything else (inbox,
        // settings) goes to the external browser.
        "mail.google.com" => {
            url.path().starts_with("/chat") || url.path().starts_with("/accounts")
        }

        // 2FA / device verification / passkeys / identity challenges.
        "gds.google.com"
        | "challenges.google.com"
        | "passwordsleakcheck-pa.googleapis.com"
        // reCAPTCHA used during sign-in challenges.
        | "www.google.com"
        | "www.recaptcha.net"
        | "recaptcha.net"
        // Google's static asset CDNs (avatars, recaptcha resources, SDK images).
        | "ssl.gstatic.com"
        | "www.gstatic.com"
        | "fonts.gstatic.com"
        | "lh3.googleusercontent.com" => true,

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(s: &str) -> Url {
        s.parse().expect("test url should parse")
    }

    #[test]
    fn whitelisted_hosts_allowed() {
        assert!(is_whitelisted(&u("https://mail.google.com/chat/u/0")));
        assert!(is_whitelisted(&u("https://chat.google.com/room/abc")));
        assert!(is_whitelisted(&u("https://accounts.google.com/signin")));
        assert!(is_whitelisted(&u("https://accounts.youtube.com/foo")));
        assert!(is_whitelisted(&u(
            "https://workspace.google.com/intl/pl/gmail/"
        )));
    }

    #[test]
    fn login_session_flow_allowed() {
        // ccTLD account domains used in the cross-domain SID cookie dance.
        assert!(is_whitelisted(&u(
            "https://accounts.google.pl/accounts/SetSID?x=1"
        )));
        assert!(is_whitelisted(&u("https://accounts.google.co.uk/signin")));
        // Gmail session setter (SetOSID) bounced through during login.
        assert!(is_whitelisted(&u(
            "https://mail.google.com/accounts/SetOSID?authuser=0"
        )));
        // Inbox itself still goes external.
        assert!(!is_whitelisted(&u(
            "https://mail.google.com/mail/u/0/#inbox"
        )));
    }

    #[test]
    fn mail_without_chat_path_blocked() {
        assert!(!is_whitelisted(&u(
            "https://mail.google.com/mail/u/0/#inbox"
        )));
        assert!(!is_whitelisted(&u("https://mail.google.com/")));
    }

    #[test]
    fn attachment_download_blocked() {
        assert!(!is_whitelisted(&u(
            "https://chat.google.com/u/0/api/get_attachment_url?id=123"
        )));
    }

    #[test]
    fn external_hosts_blocked() {
        assert!(!is_whitelisted(&u("https://github.com")));
        assert!(!is_whitelisted(&u("https://example.com/foo")));
    }

    #[test]
    fn non_http_scheme_blocked() {
        assert!(!is_whitelisted(&u("ftp://mail.google.com/chat")));
        assert!(!is_whitelisted(&u("file:///etc/passwd")));
    }

    #[test]
    fn google_2fa_hosts_allowed() {
        assert!(is_whitelisted(&u("https://gds.google.com/web/verify")));
        assert!(is_whitelisted(&u(
            "https://challenges.google.com/v1/challenge"
        )));
        assert!(is_whitelisted(&u("https://myaccount.google.com/security")));
    }

    #[test]
    fn google_static_resources_allowed() {
        assert!(is_whitelisted(&u(
            "https://ssl.gstatic.com/accounts/foo.png"
        )));
        assert!(is_whitelisted(&u(
            "https://lh3.googleusercontent.com/a/avatar"
        )));
        assert!(is_whitelisted(&u(
            "https://www.recaptcha.net/recaptcha/api2/frame"
        )));
    }
}

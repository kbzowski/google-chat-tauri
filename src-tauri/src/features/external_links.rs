use url::Url;

pub fn is_whitelisted(url: &Url) -> bool {
    if !matches!(url.scheme(), "http" | "https") {
        return false;
    }
    let Some(host) = url.host_str() else {
        return false;
    };
    match host {
        // Google Chat / Spaces - block direct attachment downloads (those go external).
        "chat.google.com" => !url.path().starts_with("/u/0/api/get_attachment_url"),

        // Gmail host - only the /chat/* subtree, everything else (inbox, settings) external.
        "mail.google.com" => url.path().starts_with("/chat"),

        // Authentication and account flows (sign-in, password, recovery, security settings).
        "accounts.google.com"
        | "accounts.youtube.com"
        | "myaccount.google.com"
        // 2FA / device verification / passkeys / identity challenges.
        | "gds.google.com"
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

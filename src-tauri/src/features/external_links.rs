use url::Url;

pub fn is_whitelisted(url: &Url) -> bool {
    if !matches!(url.scheme(), "http" | "https") {
        return false;
    }
    let Some(host) = url.host_str() else {
        return false;
    };
    match host {
        "chat.google.com" => !url.path().starts_with("/u/0/api/get_attachment_url"),
        "mail.google.com" => url.path().starts_with("/chat"),
        "accounts.google.com" | "accounts.youtube.com" => true,
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
        assert!(!is_whitelisted(&u("https://evil.google.com.fake.site")));
    }

    #[test]
    fn non_http_scheme_blocked() {
        assert!(!is_whitelisted(&u("ftp://mail.google.com/chat")));
        assert!(!is_whitelisted(&u("file:///etc/passwd")));
    }
}

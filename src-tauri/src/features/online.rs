use std::time::Duration;

pub const PING_URL: &str = "https://clients3.google.com/generate_204";
const PING_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn check_connectivity_at(url: &str) -> bool {
    let Ok(client) = reqwest::Client::builder().timeout(PING_TIMEOUT).build() else {
        return false;
    };
    client
        .get(url)
        .send()
        .await
        .map(|r| r.status().as_u16() == 204)
        .unwrap_or(false)
}

pub async fn check_connectivity() -> bool {
    check_connectivity_at(PING_URL).await
}

#[tauri::command]
pub async fn check_if_online() -> bool {
    check_connectivity().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_true_when_server_returns_204() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/generate_204")
            .with_status(204)
            .create_async()
            .await;
        let url = format!("{}/generate_204", server.url());
        assert!(check_connectivity_at(&url).await);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn returns_false_on_non_204_status() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/generate_204")
            .with_status(500)
            .create_async()
            .await;
        let url = format!("{}/generate_204", server.url());
        assert!(!check_connectivity_at(&url).await);
    }

    #[tokio::test]
    async fn returns_false_when_unreachable() {
        assert!(!check_connectivity_at("http://127.0.0.1:1/generate_204").await);
    }
}

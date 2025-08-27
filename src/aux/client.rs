use reqwest;
use std::time::Duration;

pub fn create_http_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client")
}

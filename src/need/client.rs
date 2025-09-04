use reqwest;
use std::time::Duration;

pub fn create_http_client() -> Result<reqwest::Client, ()> {
    match reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => Ok(client),
        Err(_) => Err(()),
    }
}

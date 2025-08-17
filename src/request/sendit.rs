use reqwest;
use crate::request::structure::{ApiResponse};


pub async fn send_request(url: String, client : &reqwest::Client) ->  Result<ApiResponse, Box<dyn std::error::Error>>
{
        let resp = client.get(url).send().await?;
        let json = resp.text().await?;
        let hey : ApiResponse = serde_json::from_str(&json)?;
        return Ok(hey);
}
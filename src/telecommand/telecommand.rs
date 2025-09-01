use crate::aux::api_response::ApiResponse;
use crate::aux::client;
use std::process;

#[derive(Debug)]
pub struct Telecommand {
    ip: String,
    port: String,
    client: reqwest::Client,
}

impl Telecommand {
    pub fn new(ip: String, port: String) -> Self {
        let client = match client::create_http_client() {
            Ok(client) => client,
            Err(_) => process::exit(1),
        };
        Telecommand { ip, port, client }
    }

    pub fn build_url(&self, operation: u16, key: u16, mode: u16, epg_id: u16) -> String {
        //let base_uri : String = format!("http://{}:{}/remoteControl/cmd?", ip, port);
        let ip = &self.ip;
        let port = &self.port;
        match operation {
            1 => {
                let uri: String = format!(
                    "http://{}:{}/remoteControl/cmd?operation=01&key={}&mode={}",
                    ip, port, key, mode
                );
                uri
            }
            9 => {
                let number_of_stars = 10 - epg_id.to_string().len();
                let stars = "*".repeat(number_of_stars);

                let uri: String = format!(
                    "http://{}:{}/remoteControl/cmd?operation=09&epg_id={}{}&uui=1",
                    ip, port, stars, epg_id
                );
                uri
            }
            10 => {
                let uri: String = format!("http://{}:{}/remoteControl/cmd?operation=10", ip, port);
                uri
            }
            _ => String::new(),
        }
    }

    pub async fn send_request(
        &self,
        url: String,
    ) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let resp = self.client.get(url).send().await?;
        let json = resp.text().await?;
        let hey: ApiResponse = serde_json::from_str(&json)?;
        Ok(hey)
    }
}

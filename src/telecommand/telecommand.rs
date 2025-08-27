use crate::aux::api_response::ApiResponse;
use crate::aux::client;

pub struct Telecommand {
    ip: &'static str,
    port: &'static str,
    client: reqwest::Client,
}

impl Telecommand {
    pub fn new(ip: &'static str, port: &'static str) -> Self {
        let client = client::create_http_client();
        Telecommand { ip, port, client }
    }

    pub fn build_url(
        &self,
        operation: u16,
        key: u16,
        mode: u16,
        epg_id: u16,
    ) -> Result<String, ()> {
        //let base_uri : String = format!("http://{}:{}/remoteControl/cmd?", ip, port);
        let ip = self.ip;
        let port = self.port;
        match operation {
            1 => {
                let uri: String = format!(
                    "http://{}:{}/remoteControl/cmd?operation=01&key={}&mode={}",
                    ip, port, key, mode
                );
                return Ok(uri);
            }
            9 => {
                let number_of_stars = 10 - epg_id.to_string().len();
                let stars = "*".repeat(number_of_stars);
                println!("{}", stars.len() + epg_id.to_string().len());

                let uri: String = format!(
                    "http://{}:{}/remoteControl/cmd?operation=09&epg_id={}{}&uui=1",
                    ip, port, stars, epg_id
                );
                return Ok(uri);
            }
            10 => {
                let uri: String = format!("http://{}:{}/remoteControl/cmd?operation=10", ip, port);
                return Ok(uri);
            }
            _ => {
                return Err(());
            }
        }
    }

    pub async fn send_request(
        &self,
        url: String,
    ) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let resp = self.client.get(url).send().await?;
        let json = resp.text().await?;
        let hey: ApiResponse = serde_json::from_str(&json)?;
        return Ok(hey);
    }
}

use telecommand::infos::all_infos::get_all_infos;
use telecommand::request::{client, sendit, url, structure::ApiResponse};
use telecommand::cli::choose::{choose_operation};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{

    let ip = "192.168.1.18";
    let port = "8080";
    let (operations, keys, modes) = get_all_infos();
    //infos has 3 keys that are: "operation", "modes", "keys"

    let client = client::create_http_client();

    loop{
        let operation = choose_operation(&operations);
        match operation {
            10 =>{
                let url = url::build_url(ip, port, operation, 0, 0, 0).expect("Failed to build URL");
                let hey = sendit::send_request(url, &client).await?;
                println!("Response:\n{:#?}", hey);

            },
            1 =>
            {
                println!("You chose operation: {}", operation);
                println!("Not Available yet, please choose another operation.");
            },
        }
    }
    
}

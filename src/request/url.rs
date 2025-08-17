pub fn build_url(ip : &str, port : &str, 
    operation : u16, key : u16, mode : u16, epg_id : u16) 
    -> Result<String, ()>
{
        //let base_uri : String = format!("http://{}:{}/remoteControl/cmd?", ip, port);
        match operation {
            1 => {
                let uri : String = format!("http://{}:{}/remoteControl/cmd?operation=01&key={}&mode={}", 
                    ip, port, key, mode);
                return Ok(uri);
            },
            9 => {
                let number_of_stars = 10 - epg_id.to_string().len();
                let stars = "*".repeat(number_of_stars);
                println!("{}", stars.len() + epg_id.to_string().len());

                let uri : String = format!("http://{}:{}/remoteControl/cmd?operation=09&epg_id={}{}&uui=1",
                    ip, port,stars, epg_id);
                return Ok(uri);
            },
            10 => 
            {
                let uri : String = format!("http://{}:{}/remoteControl/cmd?operation=10",
                    ip, port);
                return Ok(uri);
            }
            _ =>{
                return Err(());
            }
        }
}
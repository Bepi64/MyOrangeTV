use crate::telecommand::telecommand::Telecommand;
use std::collections::HashMap;
use std::io::{self, Write};

trait CliTelecommand {
    fn choose_operation(operations: &HashMap<&'static str, u16>) -> u16;
    fn choose_epg_id(epg_id: &HashMap<&'static str, u16>) -> u16;
    fn choose_mode(modes: &HashMap<&'static str, u16>) -> u16;
    fn choose_key(keys: &HashMap<&'static str, u16>) -> u16;
}

impl CliTelecommand for Telecommand {
    fn choose_operation(operations: &HashMap<&'static str, u16>) -> u16 {
        println!("Here the available operations:");
        for (key, value) in operations.iter() {
            println!("{}: {}", key, value);
        }
        loop {
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            let str_num = input.parse::<u16>();

            if (&str_num).is_ok() {
                let num = str_num.unwrap();
                if operations.values().any(|&v| v == num) {
                    let key = operations
                        .iter()
                        .find(|&(_, &v)| v == num)
                        .map(|(k, _)| *k)
                        .unwrap_or("Unknown operation");
                    println!("You chose operation: {}", key);
                    return num;
                } else {
                    println!("Invalid operation, please try again.");
                }
            } else if operations.contains_key(input) {
                println!("You chose operation: {}", input);
                return *operations.get(input).unwrap();
            } else {
                println!("Invalid input, please try again.");
            }
        }
    }

    fn choose_epg_id(epg_ids: &HashMap<&'static str, u16>) -> u16 {
        println!("Here the available channels:");
        for id in epg_ids.keys() {
            println!("{id}");
        }
        loop {
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut input = input.trim().to_string();
            if input.chars().all(|c| c.is_ascii_lowercase()) {
                input = input.to_ascii_uppercase();
            }

            if epg_ids.contains_key(input.as_str()) {
                println!("You chose key: {}", input);
                return *epg_ids.get(input.as_str()).unwrap() as u16;
            } else {
                println!("Invalid key, please try again.");
            }
        }
    }

    fn choose_mode(modes: &HashMap<&'static str, u16>) -> u16 {
        println!("Here the available modes:");
        for (&key, value) in modes.iter() {
            if key == "release" {
                continue; // Skip the "release" mode
            }
            println!("{}: {}", key, value);
        }
        loop {
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "release" || input == "2" {
                println!(
                    "You cannot choose the 'release' mode, it is only used after 'long press'."
                );
                continue;
            }
            let str_num = input.parse::<u16>();

            if modes.contains_key(input) {
                println!("You chose mode: {}", input);
                return *modes.get(input).unwrap();
            } else if str_num.is_ok() {
                let num = str_num.unwrap();
                if modes.values().any(|&v| v == num) {
                    let key = modes
                        .iter()
                        .find(|&(_, &v)| v == num)
                        .map(|(k, _)| *k)
                        .unwrap_or("Unknown mode");
                    println!("You chose mode: {}", key);
                    return num;
                } else {
                    println!("Invalid mode, please try again.");
                }
            } else {
                println!("Invalid input, please try again.");
            }
        }
    }

    fn choose_key(keys: &HashMap<&'static str, u16>) -> u16 {
        println!("Here the available keys:");
        for keys in keys.keys() {
            println!("{keys}");
        }
        loop {
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if keys.contains_key(input) {
                println!("You chose key: {}", input);
                return *keys.get(input).unwrap() as u16;
            } else {
                println!("Invalid key, please try again.");
            }
        }
    }
}

impl Telecommand {
    pub async fn run_cli(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (operations, keys, modes, epg_id) = crate::infos::all_infos::get_all_infos();
        loop {
            let operation = Self::choose_operation(&operations);
            match operation {
                10 => {
                    let url = self.build_url(operation, 0, 0, 0);
                    println!("{:#?}", self.send_request(url).await?);
                }
                1 => {
                    let mode = Self::choose_mode(&modes);
                    match mode {
                        0 => {
                            let key = Self::choose_key(&keys);
                            let url = self.build_url(operation, key, mode, 0);
                            println!("{:#?}", self.send_request(url).await?);
                        }
                        1 => {
                            let key = Self::choose_key(&keys);
                            let url_press = self.build_url(operation, key, mode, 0);
                            let url_release = self.build_url(operation, key, 2, 0);
                            println!("La touche va être pressé, appuyez Entrée pour relacher");
                            println!("{:#?}", self.send_request(url_press).await?);
                            let _ = io::stdin().read_line(&mut String::new());
                            println!("{:#?}", self.send_request(url_release).await?);
                        }
                        _ => {
                            println!("Invalid mode selected, please try again.");
                        }
                    }
                }
                9 => {
                    let epg_id = Self::choose_epg_id(&epg_id);
                    let url = self.build_url(operation, 0, 0, epg_id);
                    println!("{:#?}", self.send_request(url).await?);
                }
                _ => {
                    unimplemented!("Handle EPG ID operation");
                }
            }
        }
    }
}

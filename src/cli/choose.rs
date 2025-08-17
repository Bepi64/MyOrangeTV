use std::collections::HashMap;
use std::io::{self, Write};

pub fn choose_operation(operations : &HashMap<&'static str, u16>) -> u16
{
    println!("Here the available operations:");
    for (key, value) in operations.iter() {
        println!("{}: {}", key, value);
    }
    loop{

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let str_num = input.parse::<u16>();

        if (&str_num).is_ok() {
            let num = str_num.unwrap();
            if operations.values().any(|&v| v == num) {
                let key = operations.iter().find(|&(_, &v)| v == num).map(|(k, _)| *k).unwrap_or("Unknown operation");
                println!("You chose operation: {}", key);
                return num;
            } else {
                println!("Invalid operation, please try again.");
            }
        }
        else if (operations.contains_key(input)) {
            println!("You chose operation: {}", input);
            return *operations.get(input).unwrap();
        } else {
            println!("Invalid input, please try again.");
        }
    }
}

pub fn choose_key(keys : HashMap<&str, u8>) -> u8
{
    todo!()
}

pub fn choose_mode() -> u8
{
    todo!()
}

pub fn choose_epg_id() -> u8
{
    todo!()
}
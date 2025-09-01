use pnet::ipnetwork::Ipv4Network;
use pnet::{datalink::NetworkInterface, ipnetwork::IpNetwork};
use std::collections::HashSet;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_interfaces() -> Vec<NetworkInterface> {
    // Get a vector with all network interfaces found
    let all_interfaces = pnet::datalink::interfaces();

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    all_interfaces
        .into_iter()
        .filter(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .collect()
}

fn get_ipv4(interface: NetworkInterface) -> Vec<Ipv4Network> {
    interface
        .ips
        .into_iter()
        .filter_map(|ip| match ip {
            IpNetwork::V4(v4) => Some(v4),
            IpNetwork::V6(_) => None,
        })
        .collect()
}

fn get_all_ipv4(interfaces: Vec<NetworkInterface>) -> Vec<Ipv4Network> {
    interfaces.into_iter().flat_map(get_ipv4).collect()
}

fn now() -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Le temps système est avant 1970 !")
        .as_secs()
}

pub fn search_decoder(arc: Arc<Mutex<HashSet<(String, String)>>>) {
    let all_ipvs4: Vec<Ipv4Network> = get_all_ipv4(get_interfaces());

    let mut handles = vec![];

    for ipv4 in all_ipvs4 {
        let set = Arc::clone(&arc);
        let special_ip = Ipv4Addr::new(239, 255, 255, 250);
        let special_port = 1900;

        let ssdp_msg: &str = "\
M-SEARCH * HTTP/1.1\r\n\
HOST: 239.255.255.250:1900\r\n\
MAN: \"ssdp:discover\"\r\n\
MX: 2\r\n\
ST: ssdp:all\r\n\
\r\n";

        let handle = std::thread::spawn(move || {
            let start_time = now();
            let socket = std::net::UdpSocket::bind((ipv4.ip(), special_port)).unwrap();
            let _ = socket.set_read_timeout(Some(std::time::Duration::from_secs(5)));
            let _ = socket.send_to(ssdp_msg.as_bytes(), (special_ip, special_port));

            loop {
                if now() - start_time > 5 {
                    break;
                }
                let mut response = [0; 1000];
                match socket.recv_from(&mut response) {
                    Ok((_, src_ip)) => {
                        if let Ok(rep) = &str::from_utf8(&response) {
                            if let Some(line) = rep
                                .lines()
                                .find(|line| line.to_ascii_lowercase().starts_with("location:"))
                            {
                                let url = {
                                    match line.splitn(2, ':').nth(1) {
                                        Some(lineee) => lineee.trim(),
                                        None => continue,
                                    }
                                };
                                let port = {
                                    match url::Url::parse(url) {
                                        Ok(uri) => {
                                            if let Some(p) = uri.port() {
                                                p.to_string()
                                            } else {
                                                "8080".to_string()
                                            }
                                        }
                                        Err(_) => "8080".to_string(),
                                    }
                                };

                                let xml = match reqwest::blocking::get(url) {
                                    Ok(x) => {
                                        if let Ok(hey) = x.text() {
                                            hey
                                        } else {
                                            continue;
                                        }
                                    }
                                    Err(_) => continue,
                                };
                                let parsed: crate::discover::xml::Root =
                                    if let Ok(p) = serde_xml_rs::from_str(&xml) {
                                        p
                                    } else {
                                        continue;
                                    };
                                if parsed.device.friendlyName.contains("décodeur TV")
                                    && parsed.device.serviceList.service.SCPDURL.contains("Orange")
                                    && parsed.device.serviceList.service.controlURL
                                        == "/remoteControl/cmd"
                                {
                                    if let Ok(mut hey) = set.lock() {
                                        hey.insert((format!("{}", src_ip.ip()), port));
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|x| {
        let _ = x.join();
    });

    return;
}
/*
fn main() {
    dbg!(search_decoder());
}*/

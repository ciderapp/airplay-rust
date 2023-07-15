use std::io::{BufRead, BufReader};
use std::net::UdpSocket;
use std::process::Command;
use regex::Regex;

pub fn get_local_ipv4() -> Option<String> {
   let output = Command::new("ipconfig")
       .arg("/all")
       .output()
       .expect("Failed to execute ipconfig");

   let reader = BufReader::new(output.stdout.as_slice());

   let ipv4_regex = Regex::new(r#"(?i)IPv4 Address[ .]*: ([\d.]+)"#).unwrap();

   for line in reader.lines() {
       if let Ok(line) = line {
           if let Some(captures) = ipv4_regex.captures(&line) {
               if let Some(ipv4) = captures.get(1) {
                   return Some(ipv4.as_str().to_string());
               }
           }
       }
   }

   None
}

fn get_available_udp_port() -> Result<u16, std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let local_addr = socket.local_addr()?;
    Ok(local_addr.port())
}

// get the local ip address with available udp port as a string
pub fn get_local_ip_port() -> Result<String, std::io::Error> {
    let ip = get_local_ipv4().unwrap();
    let port = get_available_udp_port().unwrap();
    let ip_port = format!("{}:{}", ip, port);
    Ok(ip_port)
}


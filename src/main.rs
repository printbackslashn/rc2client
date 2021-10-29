use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //The config file
    //Change these
    let server_ip = "0.0.0.0";   //change me
    let server_port = 8000;      //change me
    let hostname = "testing";   //change me  I plan on having a set hostname as optional and using the system one in the future
    let server = format!("https://{}:{}", server_ip, server_port);
    let url = "http://localhost:8080";
    let client = reqwest::Client::builder()?
        .proxy(reqwest::Proxy::all(server)?)
        .build()?;
    let res = reqwest::Client::builder()
        .danger_disable_hostname_verification()
        .build()
        .get(&url)
        .send();
    println!("{}", res);
    Ok(())
}

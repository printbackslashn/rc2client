use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //The config file
    //Change these
    let server_ip = "0.0.0.0";   //change me
    let server_port = 8000;      //change me
    let hostname = "testing";   //change me  I plan on having a set hostname as optional and using the system one in the future
    let server = format!("https://{}:{}", server_ip, server_port);
    let url = "http://localhost:8080";
    let resp = getURL(url);
    //println!("{}", resp);
    Ok(())
}

fn getURL() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test = get("http://localhost:8000");
    Ok(())
}

fn get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(url)?;
    Ok(())
}

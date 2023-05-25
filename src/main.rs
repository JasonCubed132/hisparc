use std::collections::HashMap;

const BASE_URL: &str = "https://data.hisparc.nl/api/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(BASE_URL)?
        .json::<HashMap<String, String>>()?;

    println!("{:#?}", resp);

    println!("{:#?}", resp.get("has_data").unwrap());

    Ok(())
}

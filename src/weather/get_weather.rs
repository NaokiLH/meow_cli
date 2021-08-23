use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, env::current_dir};
const DEFAULT_LOCATION: &str = "beijing";
const KEY: &str = "SssI5SCzlGFLzb3pm";
const LANG: &str = "zh-Hans";
const URL: &str = "https://api.seniverse.com/v3/weather/now.json?";

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    location: HashMap<String, String>,
    now: HashMap<String, String>,
    last_update: String,
}
#[test]
pub fn get() -> Result<(), Box<dyn std::error::Error>> {
    let location = current_dir()?.join("location");
    let city: String;

    if !location.exists() {
        set(DEFAULT_LOCATION)?;
        city = String::from(DEFAULT_LOCATION);
    } else {
        let t = fs::read_to_string(location)?;
        city = t;
    }
    let url = format!(
        "{}key={}&location={}&language={}&unit={}",
        URL, KEY, city, LANG, 'c'
    );
    println!("{}", url);

    let resp = reqwest::blocking::get(url.as_str())?.json::<HashMap<String, Vec<Response>>>()?;
    let resp = resp.get("results").expect("no value inside!");

    let data = &resp[0].now;

    println!("{:#?}", data);

    Ok(())
}
pub fn set(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = current_dir()?.join("location");

    fs::write(file, format!("{}", city))?;

    Ok(())
}

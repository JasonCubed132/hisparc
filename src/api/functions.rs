use anyhow::{anyhow, Result};
use std::{collections::HashMap, fmt::Display};
use once_cell::sync::OnceCell;
use crate::api::structs::*;

const BASE_URL: &str = "https://data.hisparc.nl/api/";

fn get_api_urls_internal() -> Result<HashMap<String, String>> {
    let api_urls = reqwest::blocking::get(BASE_URL)?.json::<HashMap<String, String>>()?;

    let mut new_api_urls: HashMap<String, String> = HashMap::new();

    for key in api_urls.keys() {
        if key == "base_url" {
            continue;
        }
        let temp = api_urls.get(key).unwrap();
        let modded_temp = BASE_URL.to_string() + temp;
        new_api_urls.insert(key.to_string(), modded_temp);
    }

    Ok(new_api_urls)
}

static API_URLS: OnceCell<Result<HashMap<String, String>>> = OnceCell::new();

pub fn get_api_url(key: &str) -> Result<&'static str> {
    let data = API_URLS.get_or_init(|| {
        get_api_urls_internal()
    });

    match data.as_ref() {
        Ok(map) => {
            match map.get(key) {
                Some(a) => Ok(a),
                None => Err(anyhow!("Key {} not found", key))
            }
        },
        Err(e) => return Err(anyhow!(e.to_string()))
    }
}

pub fn get_station_info(
    station_number: u32,
    year: u32,
    month: u32,
    day: u32,
) -> Result<StationInfo> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(
        get_api_url("station_info")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<StationInfo>()?;
    Ok(stations)
}

pub fn get_stations_with_data(year: u32, month: u32, day: u32) -> Result<Vec<NameNumber>> {
    let mut substitions = HashMap::new();
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(
        get_api_url("stations_with_data")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_has_singles(station_number: u32, year: u32, month: u32, day: u32) -> Result<bool> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(
        get_api_url("has_singles")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<bool>()?;
    Ok(stations)
}

pub fn get_subclusters_in_cluster(cluster_number: u32) -> Result<Vec<NameNumber>> {
    let mut substitions = HashMap::new();
    substitions.insert("cluster_number".to_string(), cluster_number);

    let url = substitute_variables_with_numbers(
        get_api_url("subclusters_in_cluster")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_configuration(
    station_number: u32,
    year: u32,
    month: u32,
    day: u32,
) -> Result<StationConfig> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(
        get_api_url("configuration")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<StationConfig>()?;
    Ok(stations)
}

pub fn get_clusters() -> Result<Vec<NameNumber>> {
    let stations = reqwest::blocking::get(get_api_url("clusters")?)?
        .json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_number_of_events(
    station_number: u32,
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
) -> Result<u32> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);
    substitions.insert("hour".to_string(), hour);

    let url = substitute_variables_with_numbers(
        get_api_url("number_of_events")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<u32>()?;
    Ok(stations)
}

pub fn get_has_weather(station_number: u32, year: u32, month: u32, day: u32) -> Result<bool> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(
        get_api_url("has_weather")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<bool>()?;
    Ok(stations)
}

pub fn get_has_data(station_number: u32, year: u32, month: u32, day: u32) -> Result<bool> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url =
        substitute_variables_with_numbers(get_api_url("has_data")?, substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<bool>()?;
    Ok(stations)
}

pub fn get_clusters_in_country(country_number: u32) -> Result<Vec<NameNumber>> {
    let mut substitions = HashMap::new();
    substitions.insert("country_number".to_string(), country_number);

    let url = substitute_variables_with_numbers(
        get_api_url("clusters_in_country")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_stations_in_subcluster(subcluster_number: u32) -> Result<Vec<NameNumber>> {
    let mut substitions = HashMap::new();
    substitions.insert("subcluster_number".to_string(), subcluster_number);

    let url = substitute_variables_with_numbers(
        get_api_url("stations_in_subcluster")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_event_trace(station_number: u64, ext_timestamp: u64) -> Result<Vec<Vec<u32>>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("ext_timestamp".to_string(), ext_timestamp);

    let url = substitute_variables_with_numbers(
        get_api_url("event_trace")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<Vec<u32>>>()?;
    Ok(stations)
}

pub fn get_stations() -> Result<Vec<NameNumber>> {
    let stations = reqwest::blocking::get(get_api_url("stations")?)?
        .json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_countries() -> Result<Vec<NameNumber>> {
    let stations = reqwest::blocking::get(get_api_url("countries")?)?
        .json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_stations_with_weather(year: u32, month: u32, day: u32) -> Result<Vec<NameNumber>> {
    let mut substitions = HashMap::new();
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(
        get_api_url("stations_with_weather")?,
        substitions,
    )?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_subclusters() -> Result<Vec<NameNumber>> {
    let stations = reqwest::blocking::get(get_api_url("subclusters")?)?
        .json::<Vec<NameNumber>>()?;
    Ok(stations)
}

fn substitute_variables_with_numbers<T: Display>(
    input_str: &str,
    substitions: HashMap<String, T>,
) -> Result<String> {
    let split: Vec<&str> = input_str.split(|c| c == '{' || c == '}').collect();

    let mut output: Vec<String> = Vec::new();
    for (i, &item) in split.iter().enumerate() {
        if i % 2 == 0 {
            output.push(String::from(item));
        } else {
            match substitions.get(item) {
                Some(a) => {
                    let str_num = a.to_string();
                    output.push(str_num);
                }
                None => return Err(anyhow!("Could not find {item} in substitutions").into()),
            }
        }
    }

    Ok(output.join(""))
}

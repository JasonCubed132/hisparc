use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

const BASE_URL: &str = "https://data.hisparc.nl/api/";

#[derive(Deserialize, Debug)]
pub struct NameNumber {
    pub name: String,
    pub number: u32,
}

#[derive(Deserialize, Debug)]
pub struct Scintillator {
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct StationInfo {
    pub active: bool,
    pub altitude: Option<f32>,
    pub cluster: String,
    pub country: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub name: String,
    pub number: u32,
    pub scintillators: Vec<Scintillator>,
    pub subcluster: String,
}

#[derive(Deserialize, Debug)]
pub struct StationConfig {
    pub coinctime: f32,
    pub delay_check: f32,
    pub delay_error: f32,
    pub delay_screen: f32,
    pub detnum: f32,
    pub gps_altitude: f32,
    pub gps_latitude: f32,
    pub gps_longitude: f32,
    pub mas_ch1_adc_gain: f32,
    pub mas_ch1_adc_offset: f32,
    pub mas_ch1_comp_gain: f32,
    pub mas_ch1_comp_offset: f32,
    pub mas_ch1_current: f32,
    pub mas_ch1_gain_neg: f32,
    pub mas_ch1_gain_pos: f32,
    pub mas_ch1_inttime: f32,
    pub mas_ch1_offset_neg: f32,
    pub mas_ch1_offset_pos: f32,
    pub mas_ch1_thres_high: f32,
    pub mas_ch1_thres_low: f32,
    pub mas_ch1_voltage: f32,
    pub mas_ch2_adc_gain: f32,
    pub mas_ch2_adc_offset: f32,
    pub mas_ch2_comp_gain: f32,
    pub mas_ch2_comp_offset: f32,
    pub mas_ch2_current: f32,
    pub mas_ch2_gain_neg: f32,
    pub mas_ch2_gain_pos: f32,
    pub mas_ch2_inttime: f32,
    pub mas_ch2_offset_neg: f32,
    pub mas_ch2_offset_pos: f32,
    pub mas_ch2_thres_high: f32,
    pub mas_ch2_thres_low: f32,
    pub mas_ch2_voltage: f32,
    pub mas_common_offset: f32,
    pub mas_comp_thres_high: f32,
    pub mas_comp_thres_low: f32,
    pub mas_internal_voltage: f32,
    pub mas_max_voltage: f32,
    pub mas_reset: bool,
    pub mas_version: String,
    pub postcoinctime: f32,
    pub precoinctime: f32,
    pub reduce_data: bool,
    pub slv_ch1_adc_gain: f32,
    pub slv_ch1_adc_offset: f32,
    pub slv_ch1_comp_gain: f32,
    pub slv_ch1_comp_offset: f32,
    pub slv_ch1_current: f32,
    pub slv_ch1_gain_neg: f32,
    pub slv_ch1_gain_pos: f32,
    pub slv_ch1_inttime: f32,
    pub slv_ch1_offset_neg: f32,
    pub slv_ch1_offset_pos: f32,
    pub slv_ch1_thres_high: f32,
    pub slv_ch1_thres_low: f32,
    pub slv_ch1_voltage: f32,
    pub slv_ch2_adc_gain: f32,
    pub slv_ch2_adc_offset: f32,
    pub slv_ch2_comp_gain: f32,
    pub slv_ch2_comp_offset: f32,
    pub slv_ch2_current: f32,
    pub slv_ch2_gain_neg: f32,
    pub slv_ch2_gain_pos: f32,
    pub slv_ch2_inttime: f32,
    pub slv_ch2_offset_neg: f32,
    pub slv_ch2_offset_pos: f32,
    pub slv_ch2_thres_high: f32,
    pub slv_ch2_thres_low: f32,
    pub slv_ch2_voltage: f32,
    pub slv_common_offset: f32,
    pub slv_comp_thres_high: f32,
    pub slv_comp_thres_low: f32,
    pub slv_internal_voltage: f32,
    pub slv_max_voltage: f32,
    pub slv_reset: bool,
    pub slv_version: String,
    pub spare_bytes: f32,
    pub startmode: bool,
    pub summary: f32,
    pub timestamp: String,
    pub trig_and_or: bool,
    pub trig_external: f32,
    pub trig_high_signals: f32,
    pub trig_low_signals: f32,
    pub use_filter: bool,
    pub use_filter_threshold: bool
}

pub fn get_api_urls() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
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

pub fn get_station_info(station_number: u32, year: u32, month: u32, day: u32) -> Result<StationInfo, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("station_info").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<StationInfo>()?;
    Ok(stations)
}

pub fn get_stations_with_data(year: u32, month: u32, day: u32) -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("stations_with_data").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_has_singles(station_number: u32, year: u32, month: u32, day: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("has_singles").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<bool>()?;
    Ok(stations)
}

pub fn get_subclusters_in_cluster(cluster_number: u32) -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("cluster_number".to_string(), cluster_number);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("subclusters_in_cluster").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_configuration(station_number: u32, year: u32, month: u32, day: u32) -> Result<StationConfig, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("configuration").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<StationConfig>()?;
    Ok(stations)
}

pub fn get_clusters() -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let stations = reqwest::blocking::get(get_api_urls()?.get("clusters").unwrap())?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_number_of_events(station_number: u32, year: u32, month: u32, day: u32, hour: u32) -> Result<u32, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);
    substitions.insert("hour".to_string(), hour);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("number_of_events").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<u32>()?;
    Ok(stations)
}

pub fn get_has_weather(station_number: u32, year: u32, month: u32, day: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("has_weather").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<bool>()?;
    Ok(stations)
}

pub fn get_has_data(station_number: u32, year: u32, month: u32, day: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("has_data").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<bool>()?;
    Ok(stations)
}

pub fn get_clusters_in_country(country_number: u32) -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("country_number".to_string(), country_number);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("clusters_in_country").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_stations_in_subcluster(subcluster_number: u32) -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("subcluster_number".to_string(), subcluster_number);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("stations_in_subcluster").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_event_trace(station_number: u64, ext_timestamp: u64) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("station_number".to_string(), station_number);
    substitions.insert("ext_timestamp".to_string(), ext_timestamp);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("event_trace").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<Vec<u32>>>()?;
    Ok(stations)
}

pub fn get_stations() -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let stations = reqwest::blocking::get(get_api_urls()?.get("stations").unwrap())?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_countries() -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let stations = reqwest::blocking::get(get_api_urls()?.get("countries").unwrap())?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_stations_with_weather(year: u32, month: u32, day: u32) -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let mut substitions = HashMap::new();
    substitions.insert("year".to_string(), year);
    substitions.insert("month".to_string(), month);
    substitions.insert("day".to_string(), day);

    let url = substitute_variables_with_numbers(get_api_urls()?.get("stations_with_weather").unwrap(), substitions)?;
    let stations = reqwest::blocking::get(url)?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

pub fn get_subclusters() -> Result<Vec<NameNumber>, Box<dyn std::error::Error>> {
    let stations = reqwest::blocking::get(get_api_urls()?.get("subclusters").unwrap())?.json::<Vec<NameNumber>>()?;
    Ok(stations)
}

fn substitute_variables_with_numbers<T: Display>(input_str: &String, substitions: HashMap<String, T>) -> Result<String, Box<dyn std::error::Error>> {
    let split: Vec<&str> = input_str.split(|c| c == '{' || c == '}').collect();

    let mut output: Vec<String> = Vec::new();
    for (i, &item) in split.iter().enumerate() {
        if i % 2 == 0 {
            output.push(String::from(item));
        } else {
            match substitions.get(item) {
                Some(a) => {
                    let str_num = a.to_string();
                    output.push(String::from(str_num));
                },
                None => return Err(format!("Could not find {item} in substitutions").into())
            }
        }
    }

    Ok(output.join(""))
}

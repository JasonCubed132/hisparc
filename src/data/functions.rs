use crate::data::structs::*;
use anyhow::Result;
use chrono::{prelude::DateTime, Utc};
use reqwest::blocking::Client;

// `https://data.hisparc.nl/data/download/?data_type=events&station_events=197&start=2023-5-17&end=2023-5-20`

const BASE_URL: &str = "https://data.hisparc.nl/data/download/";

pub fn get_event_data(
    station_number: u32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<Event>> {
    let station_num_str = station_number.to_string();

    let start_string: String = format!("{}", start.format("%Y-%m-%d %H:%M:%S"));
    let end_string: String = format!("{}", end.format("%Y-%m-%d %H:%M:%S"));

    println!("{}", start_string);
    println!("{}", end_string);

    let query = vec![
        ("data_type", "events"),
        ("station_events", &station_num_str),
        ("start", &start_string),
        ("end", &end_string),
    ];

    let client = Client::new();

    let response = client.get(BASE_URL).query(&query).send()?;

    let text = response.text()?;

    let lines_iter = text.lines();

    let filtered_lines_iter = lines_iter.filter(|&x| -> bool {
        let t = String::from(x);
        !t.starts_with('#')
    });

    let parsed_lines_iter = filtered_lines_iter.map(|x| -> Result<Event> { Event::from_tsv(x) });

    let parsed_lines: Result<Vec<Event>> = parsed_lines_iter.collect();

    parsed_lines
}

// https://data.hisparc.nl/data/4/weather/?download=True&start=2020-01-23+00%3A00%3A00&end=2020-01-23+00%3A05%3A00

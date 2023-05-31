use reqwest::blocking::Client;
use anyhow::Error;

// `https://data.hisparc.nl/data/download/?data_type=events&station_events=197&start=2023-5-17&end=2023-5-20`

const BASE_URL: &str = "https://data.hisparc.nl/data/download/";

pub fn get_event_data(station_number: u32) -> Result<(), Error> {
    let station_num_str = station_number.to_string();

    let query = vec![
        ("data_type", "events"),
        ("station_events", &station_num_str),
        ("start", "2023-5-17 00:00:00"),
        ("end", "2023-5-17 00:05:00")
    ];

    let client = Client::new();

    let response = client.get(BASE_URL).query(&query).send()?;

    let text = response.text()?;

    let lines_iter = text.lines();

    let filtered_lines_iter = lines_iter.filter(|&x| -> bool {
        let t = String::from(x);
        !t.starts_with("#")
    });

    let parsed_lines_iter= filtered_lines_iter.map(|x| -> Vec<&str> {
        x.split("\t").collect()
    });

    let parsed_lines: Vec<Vec<&str>> = parsed_lines_iter.collect();

    for line in parsed_lines {
        println!("{:#?}", line);
    }

    Ok(())
}
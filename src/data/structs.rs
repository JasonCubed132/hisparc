use chrono::{DateTime, Utc, NaiveDateTime, NaiveDate, NaiveTime};
use anyhow::{Error, anyhow};

pub struct Event {
    datetime: NaiveDateTime,
    timestamp: DateTime<Utc>,
}

impl Event {
    pub fn from_tsv(input: &str) -> Result<Self, Error> {
        let split: Vec<&str> = input.split('\t').collect();

        // -1 if detector not present
        // -999 if calculation error
        // GPS Date
        // GPS Time
        // Unix Timestamp
        // Unix Timestamp NS
        // Pulseheights x4
        // Integral x4
        // Number of Mips x4
        // Arrival Times x4
        // Trigger Time NS
        // Zenith
        // Azimuth

        let gps_date = NaiveDate::parse_from_str(split[0], "%d/%m/%Y")?;
        let gps_time = NaiveTime::parse_from_str(split[1], "%H:%M:%S")?;
        let gps_timestamp = NaiveDateTime::new(gps_date, gps_time);

        let unix_timestamp_s = split[2].parse::<i64>()?;
        let unix_timestamp_ns = split[3].parse::<u32>()?;

        let unix_timestamp = match NaiveDateTime::from_timestamp_opt(unix_timestamp_s, unix_timestamp_ns) {
            Some(t) => t,
            None => return Err(anyhow!("Time too far in the future!")),
        }.and_utc();

        todo!();

        Ok(Self {
            datetime: gps_timestamp,
            timestamp: unix_timestamp
        })

    }
}
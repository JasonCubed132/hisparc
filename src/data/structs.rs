use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[derive(Debug)]
pub struct Event {
    datetime: NaiveDateTime,
    timestamp: DateTime<Utc>,
    pulseheights: DetectorDataGroup<u32>,
    integrals: DetectorDataGroup<u32>,
    mips_numbers: DetectorDataGroup<f32>,
    arrival_times: DetectorDataGroup<f32>,
    trigger_time: f32,
    reconstructed_angle: Option<AxialCoord>,
}

#[derive(Debug)]
pub struct DetectorDataGroup<T> {
    detector_1: Option<T>,
    detector_2: Option<T>,
    detector_3: Option<T>,
    detector_4: Option<T>,
}

#[derive(Debug)]
pub struct AxialCoord {
    zenith: f32,
    azimuth: f32,
}

impl Event {
    pub fn from_tsv(input: &str) -> Result<Self> {
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

        let gps_date = NaiveDate::parse_from_str(split[0], "%Y-%m-%d").context(format!(
            "attempted to parse {} using \"%d-%m-%Y\"",
            split[0]
        ))?;
        let gps_time = NaiveTime::parse_from_str(split[1], "%H:%M:%S").context(format!(
            "attempted to parse {} using \"%H:%M:%S\"",
            split[1]
        ))?;
        let gps_timestamp = NaiveDateTime::new(gps_date, gps_time);

        let unix_timestamp_s = split[2]
            .parse::<i64>()
            .context(format!("attempted to parse {} as i64", split[2]))?;
        let unix_timestamp_ns = split[3]
            .parse::<u32>()
            .context(format!("attempted to parse {} as u32", split[2]))?;

        let unix_timestamp =
            match NaiveDateTime::from_timestamp_opt(unix_timestamp_s, unix_timestamp_ns) {
                Some(t) => t,
                None => {
                    return Err(anyhow!(
                        "Time {} {} is too far in the future!",
                        unix_timestamp_s,
                        unix_timestamp_ns
                    ))
                }
            }
            .and_utc();

        let pulseheights_raw: Vec<Option<u32>> =
            parse_list(split[4..8].to_vec()).context("parsing pulseheights")?;
        let integrals_raw: Vec<Option<u32>> =
            parse_list(split[8..12].to_vec()).context("parsing integrals")?;
        let mips_numbers_raw: Vec<Option<f32>> =
            parse_list(split[12..16].to_vec()).context("parsing mips_numbers")?;
        let arrival_times_raw: Vec<Option<f32>> =
            parse_list(split[16..20].to_vec()).context("parsing arrival_times")?;

        let pulseheights =
            map_list_of_four_to_detector_group(pulseheights_raw).context("mapping pulseheights")?;
        let integrals =
            map_list_of_four_to_detector_group(integrals_raw).context("parsing integrals")?;
        let mips_numbers =
            map_list_of_four_to_detector_group(mips_numbers_raw).context("parsing mips_numbers")?;
        let arrival_times = map_list_of_four_to_detector_group(arrival_times_raw)
            .context("parsing arrival_times")?;

        let trigger_time: f32 = split[20]
            .parse()
            .context(format!("parsing {} as f32 for trigger time", split[20]))?;

        let angles: Vec<Option<f32>> =
            parse_list(split[21..23].to_vec()).context("parsing angles")?;

        let angle;
        if angles[0] == None || angles[1] == None {
            angle = None;
        } else {
            angle = Some(AxialCoord {
                zenith: angles[0].unwrap(),
                azimuth: angles[1].unwrap(),
            });
        }

        Ok(Self {
            datetime: gps_timestamp,
            timestamp: unix_timestamp,
            pulseheights,
            integrals,
            mips_numbers,
            arrival_times,
            trigger_time,
            reconstructed_angle: angle,
        })
    }
}

fn parse_list<T: FromStr>(input_vec: Vec<&str>) -> Result<Vec<Option<T>>>
where
    <T as FromStr>::Err: Send + Sync + std::error::Error + 'static,
{
    let mut result: Vec<Option<T>> = Vec::new();

    for &item in &input_vec {
        match item {
            "-1" | "-999" => {
                result.push(None);
            }
            a => {
                let parsed = a.parse::<T>();

                match parsed {
                    Ok(e) => {
                        result.push(Some(e));
                    }
                    Err(e) => {
                        let err: Error = e.into();
                        return Err(err).context(format!(
                            "attempted to parse {} from vec {:?}",
                            item, input_vec
                        ));
                    }
                }
            }
        }
    }

    Ok(result)
}

fn map_list_of_four_to_detector_group<T: Clone + Debug>(
    input_vec: Vec<Option<T>>,
) -> Result<DetectorDataGroup<T>> {
    if input_vec.len() != 4 {
        Err(anyhow!("Input vector {:?} is not of length 4!", input_vec))
    } else {
        Ok(DetectorDataGroup {
            detector_1: input_vec[0].clone(),
            detector_2: input_vec[1].clone(),
            detector_3: input_vec[2].clone(),
            detector_4: input_vec[3].clone(),
        })
    }
}

use hisparc::api::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_urls = get_api_urls()?;

    println!("{:#?}", api_urls);

    // println!("{:#?}", api_urls.get("has_data").unwrap());

    // let stations = get_stations()?;
    // println!("{:#?}", stations[0]);

    // let station_info = get_station_info(4, 2023, 5, 23)?;
    // println!("{:#?}", station_info);

    // let stations_with_data = get_stations_with_data(2023, 5, 23)?;
    // println!("{:#?}", stations_with_data);

    // let has_singles = get_has_singles(14006, 2023, 5, 23)?;
    // println!("{:#?}", has_singles);

    // let subclusters = get_subclusters_in_cluster(1000)?;
    // println!("{:#?}", subclusters);

    // let config = get_configuration(14006, 2023, 5, 23)?;
    // println!("{:#?}", config);

    // let clusters = get_clusters()?;
    // println!("{:#?}", clusters);

    // let num_events = get_number_of_events(14006, 2023, 5, 23, 0)?;
    // println!("{:#?}", num_events);

    // let has_weather = get_has_weather(14006, 2023, 5, 23)?;
    // println!("{:#?}", has_weather);

    // let has_data = get_has_data(14006, 2023, 5, 23)?;
    // println!("{:#?}", has_data);

    // let clusters_in_country = get_clusters_in_country(10000)?;
    // println!("{:#?}", clusters_in_country);

    // let stations_in_subcluster = get_stations_in_subcluster(0)?;
    // println!("{:#?}", stations_in_subcluster);

    Ok(())
}

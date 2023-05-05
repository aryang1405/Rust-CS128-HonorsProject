use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use csv::ReaderBuilder;
use chrono::{NaiveDateTime, Utc};
use ts_rs::{TimeSeries, TimeSeriesData, TimeSeriesError};
use knn::{Knn, Weighting};

fn main() -> Result<(), Box<dyn Error>> {
    // Read electricity consumption data from CSV file
    let file = File::open("electricity_consumption.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(BufReader::new(file));
    let mut data = vec![];
    for result in reader.records() {
        let record = result?;
        let datetime = NaiveDateTime::parse_from_str(record[0].as_ref(), "%Y-%m-%dT%H:%M:%SZ")?;
        let consumption = record[1].parse::<f64>()?;
        data.push((datetime.timestamp() as u64, consumption));
    }

    // Create time series data from consumption data
    let mut time_series_data = TimeSeriesData::new();
    for (timestamp, consumption) in data {
        time_series_data.add_entry(timestamp, consumption)?;
    }

    // Resample time series data to hourly frequency
    let resampled_time_series_data = time_series_data.resample("1H")?;

    // Split time series data into train and test sets
    let train_data = resampled_time_series_data.slice(None, Some(-24 * 7));
    let test_data = resampled_time_series_data.slice(Some(-24 * 7), None);

    // Train K-Nearest Neighbors model on train data
    let mut knn = Knn::new(train_data, 5, Weighting::Distance)?;
    knn.train()?;

    // Make predictions on test data
    let mut predictions = vec![];
    for (timestamp, _) in test_data.timestamps() {
        let consumption = test_data.get_entry(timestamp)?;
        let prediction = knn.predict(consumption)?;
        let datetime = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        predictions.push((Utc.from_utc_datetime(&datetime), prediction));
    }

    // Print predictions
    for (datetime, prediction) in predictions {
        println!("{}: {}", datetime, prediction);
    }

    Ok(())
}


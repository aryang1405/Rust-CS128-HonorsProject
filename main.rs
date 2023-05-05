use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::HashMap;

struct Observation {
    features: Vec<f64>,
    label: f64,
}

fn read_data(file_path: &str) -> Result<Vec<Observation>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines().skip(1) {
        let line = line?;
        let values: Vec<f64> = line.split(';')
                                   .map(|s| s.parse::<f64>().unwrap())
                                   .collect();
        let obs = Observation {
            features: values[..-1].to_vec(),
            label: values.last().unwrap().clone(),
        };
        data.push(obs);
    }

    Ok(data)
}

fn predict_knn(k: usize, train_data: &[Observation], test_point: &[f64]) -> f64 {
    let mut dist_map = HashMap::new();
    for obs in train_data {
        let dist = euclidean_distance(&obs.features, test_point);
        dist_map.insert(dist, obs.label);
    }

    let mut k_neighbors = Vec::new();
    for dist in dist_map.keys().sorted().take(k) {
        k_neighbors.push(dist_map.get(dist).unwrap());
    }

    *k_neighbors.iter().sum::<f64>() / k as f64
}

fn euclidean_distance(point1: &[f64], point2: &[f64]) -> f64 {
    let sum_squares = point1.iter()
                            .zip(point2.iter())
                            .map(|(&x, &y)| (x - y).powi(2))
                            .sum::<f64>();
    sum_squares.sqrt()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_data("household_power_consumption.txt")?;
    let n_samples = data.len();
    let train_size = (n_samples as f64 * 0.8).round() as usize;

    let mut train_data = Vec::new();
    let mut test_data = Vec::new();
    for (i, obs) in data.iter().enumerate() {
        if i < train_size {
            train_data.push(obs.clone());
        } else {
            test_data.push(obs.clone());
        }
    }

    let mut correct_predictions = 0;
    for test_point in &test_data {
        let predicted = predict_knn(5, &train_data, &test_point.features);
        if (predicted - test_point.label).abs() < 0.1 {
            correct_predictions += 1;
        }
    }

    let accuracy = correct_predictions as f64 / test_data.len() as f64;
    println!("Accuracy: {:.2}%", accuracy * 100.0);

    Ok(())
}

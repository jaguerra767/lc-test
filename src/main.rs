use std::{thread, time::Duration};

use anyhow::Result;
use libra::scale::Scale;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

fn main() {
    const PHIDGET_SN: i32 = 716692;
    //const PHIDGET_SN: i32 = 716800;
    const COEFFICIENTS: [f64; 4] = [
        5286438.30017923,
        -5090856.07529369,
        -5119668.03727095,
        -4916037.57602048,
    ];

    let scale = Scale::new(PHIDGET_SN, 0.0, COEFFICIENTS).connect();

    loop {
        let mut weights: Vec<f64> = (0..50).map(|_| scale.get_weight().unwrap()).collect();

        let weight = median(weights.as_mut_slice());

        let client = Client::new();
        match send_request(&client, weight) {
            Ok(_) => {
                thread::sleep(Duration::from_secs(600));
            }
            Err(e) => {
                eprint!("Error occured: {}", e);
                break;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Weight {
    weight: f64,
    ichibu_id: u64,
}

fn send_request(client: &Client, weight: f64) -> Result<()> {
    let payload = Weight {
        weight,
        ichibu_id: 1234,
    };

    let _ = client
        .post("https://us-central1-ichibu-testing.cloudfunctions.net/sensor")
        .json(&payload)
        .send()?;
    Ok(())
}

fn median(range: &mut [f64]) -> f64 {
    range.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let middle = range.len() / 2;
    range[middle]
}

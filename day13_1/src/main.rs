use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let earliest_timestamp = lines[0].parse::<i64>().unwrap();
    let bus_ids = lines[1]
        .split(',')
        .filter(|bus_timestamp| *bus_timestamp != "x")
        .map(|bus_timestamp| bus_timestamp.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut best_bus = (
        0,        // bus id
        i64::MAX, // calculated bus timestamp
    );

    for bus_id in bus_ids {
        let bus_timestamp = bus_id;
        let calculated_bus_timestamp = get_earliest_timestamp(earliest_timestamp, bus_timestamp);

        // Check if this bus is better / we have to wait less
        if calculated_bus_timestamp < best_bus.1 {
            best_bus.0 = bus_id;
            best_bus.1 = calculated_bus_timestamp;
        }
    }

    let minutes_to_wait = best_bus.1 - earliest_timestamp;

    println!(
        "{} * {} = {}",
        best_bus.0,
        minutes_to_wait,
        best_bus.0 * minutes_to_wait
    );

    Ok(())
}

fn get_earliest_timestamp(target_timestamp: i64, bus_timestamp: i64) -> i64 {
    let multiplier = (target_timestamp as f64 / bus_timestamp as f64).ceil() as i64;

    bus_timestamp * multiplier
}

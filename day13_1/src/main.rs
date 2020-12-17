use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let earliest_timestamp = lines[0].parse::<i32>().unwrap();
    let bus_ids = lines[1]
        .split(',')
        .filter(|bus_timestamp| *bus_timestamp != "x")
        .map(|bus_timestamp| bus_timestamp.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut best_bus = (
        0,        // bus id
        i32::MAX, // calculated_bus_timestamp
    );

    for bus_id in bus_ids {
        let bus_timestamp = bus_id;
        let multiples = (earliest_timestamp as f64 / bus_timestamp as f64).ceil() as i32;
        let calculated_bus_timestamp = bus_timestamp * multiples;

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

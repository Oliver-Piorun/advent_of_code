use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let bus_ids = lines[1].split(',').collect::<Vec<&str>>();

    let mut buses = Vec::new();

    for (i, bus_id) in bus_ids.iter().enumerate() {
        if *bus_id != "x" {
            let bus_id = bus_id.parse::<i64>().unwrap();
            let timestamp_offset = i;
            let bus = (bus_id, timestamp_offset);

            buses.push(bus);
        }
    }

    let earliest_timestamp_threshold = 100_000_000_000_000;
    let first_bus = buses[0];
    let mut current_timestamp = get_earliest_timestamp(earliest_timestamp_threshold, first_bus.0);
    let mut timestamp_step = first_bus.0;
    let mut bus_id_index = 1;

    loop {
        let bus = buses[bus_id_index];
        let bus_id = bus.0;
        let target_timestamp = current_timestamp + bus.1 as i64;
        let calculated_bus_timestamp = get_earliest_timestamp(target_timestamp, bus_id);

        // Check if the calculated bus timestamp is fitting the current timestamp
        if calculated_bus_timestamp - bus.1 as i64 == current_timestamp {
            // Check if we processed all bus ids
            if bus_id_index + 1 >= buses.len() {
                println!("{}", current_timestamp);
                break;
            }

            // Update the timestamp step based on the bus id
            timestamp_step *= bus_id;

            // Proceed with the next bus
            bus_id_index += 1;
        }

        current_timestamp += timestamp_step;
    }

    Ok(())
}

fn get_earliest_timestamp(timestamp: i64, bus_id: i64) -> i64 {
    let multiplier = (timestamp as f64 / bus_id as f64).ceil() as i64;

    bus_id * multiplier
}

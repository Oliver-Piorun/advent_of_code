use std::{fs, io};

fn main() -> io::Result<()> {
    // Read the input
    let input = fs::read_to_string("input")?;

    // Split at "," to get the horizontal positions
    let horizontal_positions = input
        .split(',')
        .map(|horizontal_position_str| horizontal_position_str.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    part1(&mut horizontal_positions.clone());
    part2(&horizontal_positions);

    Ok(())
}

fn part1(horizontal_positions: &mut [u32]) {
    // Sort horizontal positions in ascending order
    horizontal_positions.sort_unstable();

    // Get the median
    let median = horizontal_positions[horizontal_positions.len() / 2];

    let mut total_min_required_fuel = 0;

    // Iterate over each horizontal position
    for horizontal_position in horizontal_positions {
        // The absolute distance to the median is equal to the minimum required fuel
        // because the median is the center of attraction
        let absolute_distance = horizontal_position.abs_diff(median);
        let min_required_fuel = absolute_distance;
        total_min_required_fuel += min_required_fuel;
    }

    println!("part1: {total_min_required_fuel}");
}

fn part2(horizontal_positions: &[u32]) {
    // Get the sum of all horizontal positions
    let sum = horizontal_positions.iter().sum::<u32>();

    // Calculate the average and floor/ceil it
    let average = sum as f32 / horizontal_positions.len() as f32;
    let average_floor = average.floor() as u32;
    let average_ceil = average.ceil() as u32;

    let mut total_required_fuel_floor = 0;
    let mut total_required_fuel_ceil = 0;

    // Iterate over each horizontal position
    for horizontal_position in horizontal_positions {
        // The absolute distance to the floored/ceiled average is equal to the minimum required fuel
        let absolute_distance_floor = horizontal_position.abs_diff(average_floor);
        let required_fuel_floor = (absolute_distance_floor * (absolute_distance_floor + 1)) / 2;
        total_required_fuel_floor += required_fuel_floor;

        let absolute_distance_ceil = horizontal_position.abs_diff(average_ceil);
        let required_fuel_ceil = (absolute_distance_ceil * (absolute_distance_ceil + 1)) / 2;
        total_required_fuel_ceil += required_fuel_ceil;
    }

    // Figure out which average leads to less fuel
    let total_min_required_fuel = total_required_fuel_floor.min(total_required_fuel_ceil);

    println!("part2: {total_min_required_fuel}");
}

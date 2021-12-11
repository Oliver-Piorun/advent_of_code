use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut numbers: Vec<u8> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<u8>().unwrap())
        .collect();
    numbers.sort_unstable();

    let mut num_one_jolt_difference = 0;
    let mut num_three_jolt_difference = 0;

    for (i, number) in numbers.iter().enumerate() {
        let jolt_difference = if i == 0 {
            *number
        } else {
            number - numbers[i - 1]
        };

        if jolt_difference == 1 {
            num_one_jolt_difference += 1;
        } else if jolt_difference == 3 {
            num_three_jolt_difference += 1;
        }
    }

    // Last adapter to device difference
    num_three_jolt_difference += 1;

    println!(
        "{} * {} = {}",
        num_one_jolt_difference,
        num_three_jolt_difference,
        num_three_jolt_difference * num_one_jolt_difference
    );

    Ok(())
}

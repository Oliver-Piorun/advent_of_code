use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let numbers: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    // Iterate over each number and skip the preamble
    for (i, number) in numbers.iter().enumerate().skip(25) {
        let mut found = false;

        // Find a fitting pair of numbers in the previous 25 numbers (relative position)
        for j in i - 25..=i - 1 {
            for k in i - 25..=i - 1 {
                // Make sure the numbers are not equal and actually add up to the current number
                if numbers[j] != numbers[k] && (numbers[j] + numbers[k] == *number) {
                    found = true;
                }
            }
        }

        // Check if we did not find a fitting pair
        if !found {
            println!("{number}");
            break;
        }
    }

    Ok(())
}

use std::{collections::HashSet, error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let numbers: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let outlier_number = find_outlier_number(&numbers).unwrap();
    let mut unique_numbers = HashSet::new();

    // Try to find a subset sum which is equal to the outlier number
    for lower_boundary in 0..=numbers.len() - 2 {
        let mut sum = 0;

        for upper_boundary in lower_boundary + 1..=numbers.len() - 1 {
            // Check if number was not already used
            if !unique_numbers.contains(&numbers[upper_boundary]) {
                unique_numbers.insert(numbers[upper_boundary]);

                sum += numbers[upper_boundary];

                if sum == outlier_number {
                    let mut numbers_subset = numbers[lower_boundary + 1..=upper_boundary].to_vec();
                    numbers_subset.sort_unstable();

                    println!(
                        "{}",
                        numbers_subset[0] + numbers_subset[numbers_subset.len() - 1]
                    );

                    return Ok(());
                }
            } else {
                break;
            }
        }

        unique_numbers.clear();
    }

    Ok(())
}

fn find_outlier_number(numbers: &[i64]) -> Option<i64> {
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
            return Some(*number);
        }
    }

    None
}

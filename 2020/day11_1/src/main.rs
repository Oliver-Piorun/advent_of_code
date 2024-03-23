use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut characters: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut result = characters.clone();

    loop {
        let mut changed_seat = false;

        // Iterate over all characters
        for i in 0..characters.len() {
            for j in 0..characters[i].len() {
                let character = characters[i][j];

                if character == 'L' {
                    // Change empty seat to an occupied seat if there are no directly adjacent occupied seats
                    if get_num_occupied_seats(i, j, &characters) == 0 {
                        result[i][j] = '#';
                        changed_seat = true;
                    } else {
                        // Keep empty seat
                        result[i][j] = 'L';
                    }
                } else if character == '#' {
                    // Change occupied seat to an empty seat if there are more than 4 directly adjacent occupied seats
                    if get_num_occupied_seats(i, j, &characters) >= 4 {
                        result[i][j] = 'L';
                        changed_seat = true;
                    } else {
                        // Keep occupied seat
                        result[i][j] = '#';
                    }
                } else {
                    // Keep floor
                    result[i][j] = '.';
                }
            }
        }

        // Use the result of this round as a basis for the next one
        characters.clone_from(&result);

        // Stop if no seat was changed
        if !changed_seat {
            break;
        }
    }

    let num_occupied_seats = characters
        .iter()
        .flatten()
        .fold(0, |accumulator, character| {
            if *character == '#' {
                accumulator + 1
            } else {
                accumulator
            }
        });

    println!("{num_occupied_seats}");

    Ok(())
}

fn get_num_occupied_seats(i: usize, j: usize, characters: &[Vec<char>]) -> i32 {
    let mut num_occupied_seats = 0;

    // Take a look at the directly adjacent characters
    for k in -1..=1 {
        for l in -1..=1 {
            // Skip the character for which we are checking
            if k == 0 && l == 0 {
                continue;
            }

            let row_index = i as isize + k;
            let column_index = j as isize + l;

            // Skip the character if it is out of bounds
            if row_index < 0
                || row_index >= characters.len() as isize
                || column_index < 0
                || column_index >= characters[row_index as usize].len() as isize
            {
                continue;
            }

            if characters[row_index as usize][column_index as usize] == '#' {
                num_occupied_seats += 1;
            }
        }
    }

    num_occupied_seats
}

use std::{cmp::max, error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

static DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),  // Top
    (1, 0),   // Bottom
    (0, -1),  // Left
    (0, 1),   // Right
    (-1, -1), // Top left
    (-1, 1),  // Top right
    (1, -1),  // Bottom left
    (1, 1),   // Bottom right
];

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
                    // Change empty seat to an occupied seat if there are no occupied seats in the 8 directions
                    if get_num_occupied_seats(i, j, &characters) == 0 {
                        result[i][j] = '#';
                        changed_seat = true;
                    } else {
                        result[i][j] = 'L';
                    }
                } else if character == '#' {
                    // Change occupied seat to an empty seat if there are more than 5 occupied seats in the 8 directions
                    if get_num_occupied_seats(i, j, &characters) >= 5 {
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
        characters = result.clone();

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

    println!("{}", num_occupied_seats);

    Ok(())
}

fn get_num_occupied_seats(i: usize, j: usize, characters: &[Vec<char>]) -> i32 {
    let mut num_occupied_seats = 0;

    let max = max(characters.len(), characters[0].len());

    // Iterate over all directions
    for (k, l) in DIRECTIONS {
        // Use the factor to follow the current direction
        for factor in 1..max as isize {
            let row_index = i as isize + k * factor;
            let column_index = j as isize + l * factor;

            if row_index < 0
                || row_index >= characters.len() as isize
                || column_index < 0
                || column_index >= characters[row_index as usize].len() as isize
            {
                // Continue with the next direction, because the character is out of bounds
                break;
            }

            if characters[row_index as usize][column_index as usize] == '#' {
                num_occupied_seats += 1;

                // Continue with the next direction, because an occupied seat has been found
                break;
            } else if characters[row_index as usize][column_index as usize] == 'L' {
                // Continue with the next direction, because an empty seat has been found
                break;
            }
        }
    }

    num_occupied_seats
}

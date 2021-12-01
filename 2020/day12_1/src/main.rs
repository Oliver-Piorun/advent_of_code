use core::panic;
use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_u8(value: u8) -> Direction {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Unhandled value! ({value})"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::East;

    for line in lines {
        let action = &line[0..1];
        let value = line[1..].parse::<i32>().unwrap();

        match action {
            "N" => y += value, // Move ship north
            "S" => y -= value, // Move ship south
            "E" => x += value, // Move ship east
            "W" => x -= value, // Move ship west
            "L" => {
                // Rotate ship counter-clockwise
                let num_direction_changes = value / 90;

                // Wrap around the directions
                let mut new_direction_value = (direction as i8 - num_direction_changes as i8) % 4;

                if new_direction_value < 0 {
                    new_direction_value += 4;
                }

                direction = Direction::from_u8(new_direction_value as u8);
            }
            "R" => {
                // Rotate ship clockwise
                let num_direction_changes = value / 90;

                // Wrap around the directions
                let new_direction_value = (direction as u8 + num_direction_changes as u8) % 4;

                direction = Direction::from_u8(new_direction_value);
            }
            "F" => {
                // Move ship torwards the direction
                match direction {
                    Direction::North => y += value,
                    Direction::South => y -= value,
                    Direction::East => x += value,
                    Direction::West => x -= value,
                };
            }
            _ => panic!("Unhandled action! ({action})"),
        }
    }

    // Print manhattan distance
    println!("{} + {} = {}", x.abs(), y.abs(), x.abs() + y.abs());

    Ok(())
}

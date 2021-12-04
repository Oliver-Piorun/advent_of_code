use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(PartialEq, Copy, Clone)]
enum RatingType {
    OxygenGenerator,
    CO2Scrubber,
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input")?;

    // Read all lines
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    part1(&lines);
    part2(&lines);

    Ok(())
}

fn part1(lines: &[String]) {
    let num_horizontal_bits = lines[0].len();

    let mut gamma_rate = 0;

    // Take a look at each horizontal bit
    for horizontal_bit_index in 0..num_horizontal_bits {
        // Get the most common bit
        let most_common_bit = get_most_common_vertical_bit(lines, horizontal_bit_index);

        if most_common_bit == '0' {
            // We do not have to set a bit here
            continue;
        }

        // Set the bit at the horizontal bit index
        gamma_rate |= 1 << (num_horizontal_bits - 1 - horizontal_bit_index);
    }

    let bit_mask = !(!0 << num_horizontal_bits);

    // Invert the gamma rate and clear all not relevant bits with a bit mask
    let epsilon_rate = !gamma_rate & bit_mask;

    println!("part1: {}", gamma_rate * epsilon_rate);
}

fn part2(lines: &[String]) {
    // Get the oxygen generator rating
    let oxygen_generator_rating = get_rating(lines, RatingType::OxygenGenerator);

    // Get the CO2 scrubber rating
    let co2_scrubber_rating = get_rating(lines, RatingType::CO2Scrubber);

    println!("part2: {}", oxygen_generator_rating * co2_scrubber_rating);
}

fn get_most_common_vertical_bit(lines: &[String], horizontal_bit_index: usize) -> char {
    let num_vertical_bits = lines.len() as i32;

    let mut num_unset_bits = 0;
    let mut num_set_bits = 0;

    // Take a look at each vertical bit
    for (vertical_bit_index, line) in lines.iter().enumerate() {
        // Get the horizontal bit
        let horizontal_bit = line.chars().nth(horizontal_bit_index).unwrap();

        match horizontal_bit {
            '0' => num_unset_bits += 1,
            '1' => num_set_bits += 1,
            _ => panic!("Invalid horizontal bit!"),
        };

        let num_remaining_vertical_bits = num_vertical_bits - vertical_bit_index as i32 - 1;

        if num_unset_bits > num_set_bits + num_remaining_vertical_bits {
            // Check if enough bits are unset to call them more common
            return '0';
        } else if num_set_bits >= num_unset_bits + num_remaining_vertical_bits {
            // Check if enough bits are set to call them more common
            return '1';
        }
    }

    panic!("We should never land here!")
}

fn get_rating(lines: &[String], rating_type: RatingType) -> i32 {
    let mut rating_vec: Vec<String> = lines.to_vec();
    let mut horizontal_bit_index = 0;

    while rating_vec.len() != 1 {
        // Get the most common vertical bit
        let most_common_vertical_bit =
            get_most_common_vertical_bit(&rating_vec, horizontal_bit_index);

        // Filter by most common vertical bit and rating type
        filter(
            &mut rating_vec,
            horizontal_bit_index,
            most_common_vertical_bit,
            rating_type,
        );

        // Increase horizontal bit index
        horizontal_bit_index += 1;
    }

    // Convert binary string to an integer
    i32::from_str_radix(&rating_vec[0], 2).unwrap()
}

fn filter(
    lines: &mut Vec<String>,
    horizontal_bit_index: usize,
    most_common_vertical_bit: char,
    rating_type: RatingType,
) {
    let horizontal_bit_to_retain = match most_common_vertical_bit {
        '0' => match rating_type {
            RatingType::OxygenGenerator => '0',
            RatingType::CO2Scrubber => '1',
        },
        '1' => match rating_type {
            RatingType::OxygenGenerator => '1',
            RatingType::CO2Scrubber => '0',
        },
        _ => panic!("Invalid most common vertical bit!"),
    };

    lines
        .retain(|line| line.chars().nth(horizontal_bit_index).unwrap() == horizontal_bit_to_retain);
}

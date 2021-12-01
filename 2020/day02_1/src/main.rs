use std::io::{self, BufReader};
use std::{fs::File, io::BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut num_valid_passwords = 0;

    for line in lines {
        let line_split: Vec<&str> = line.split(' ').collect();

        // min, max
        let min_max = line_split[0];
        let min_max_split: Vec<&str> = min_max.split('-').collect();
        let min = min_max_split[0].parse::<i32>().unwrap();
        let max = min_max_split[1].parse::<i32>().unwrap();

        // letter
        let mut letter = line_split[1].to_string();
        letter.pop(); // Remove ":"

        // password
        let password = line_split[2];

        let mut num_occurences = 0;

        // Count number of letter occurences
        for char in password.chars() {
            if char.to_string() == letter {
                num_occurences += 1;
            }
        }

        // Check if password is valid
        if num_occurences >= min && num_occurences <= max {
            println!("{min} {max} {letter} {password}");
            num_valid_passwords += 1;
        }
    }

    println!("{num_valid_passwords}");

    Ok(())
}

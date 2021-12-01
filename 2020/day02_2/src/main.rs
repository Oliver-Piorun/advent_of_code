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

        // first_index, second_index
        let indices = line_split[0];
        let indices: Vec<&str> = indices.split('-').collect();
        let first_index = indices[0].parse::<usize>().unwrap();
        let second_index = indices[1].parse::<usize>().unwrap();

        // letter
        let mut letter = line_split[1].to_string();
        letter.pop(); // Remove ":"

        // password
        let password = line_split[2];

        // Check if password is valid
        if (password.chars().nth(first_index - 1).unwrap().to_string() == letter)
            ^ (password.chars().nth(second_index - 1).unwrap().to_string() == letter)
        {
            println!("{first_index} {second_index} {letter} {password}");
            num_valid_passwords += 1;
        }
    }

    println!("{num_valid_passwords}");

    Ok(())
}

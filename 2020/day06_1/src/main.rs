use std::{collections::HashSet, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    let groups: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut num_yes = 0;

    for group in groups {
        let mut questions = HashSet::new();

        for character in group.chars() {
            // Skip carriage return (CR) and line feed (LF)
            if character != '\r' && character != '\n' {
                questions.insert(character);
            }
        }

        num_yes += questions.len();
    }

    println!("{num_yes}");

    Ok(())
}

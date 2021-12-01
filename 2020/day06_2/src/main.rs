use std::{collections::HashMap, error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    let groups: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut num_yes = 0;

    for group in groups {
        let mut questions = HashMap::new();
        let members: Vec<&str> = group.split("\r\n").collect();

        for member in &members {
            // Add questions and their number of occurences
            for question in member.chars() {
                *questions.entry(question).or_insert(0) += 1;
            }
        }

        // Add number of yes answers to all questions
        num_yes += questions.values().fold(0, |accumulator, &occurences| {
            if occurences == members.len() {
                accumulator + 1
            } else {
                accumulator
            }
        });
    }

    println!("{num_yes}");

    Ok(())
}

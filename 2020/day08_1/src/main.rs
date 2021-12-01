use std::{collections::HashSet, error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut executed_instruction_indices = HashSet::new();
    let mut instruction_index: i32 = 0;
    let mut accumulator = 0;

    loop {
        let instruction = &lines[instruction_index as usize];

        if !executed_instruction_indices.contains(&instruction_index) {
            executed_instruction_indices.insert(instruction_index);
        } else {
            break;
        }

        let split: Vec<&str> = instruction.split(' ').collect();
        let operation = split[0];
        let argument = split[1].parse::<i32>()?;

        match operation {
            "acc" => {
                accumulator += argument;
                instruction_index += 1;
            }
            "jmp" => instruction_index += argument,
            "nop" => instruction_index += 1,
            _ => panic!("Unhandled operation! ({operation})"),
        }
    }

    println!("{accumulator}");

    Ok(())
}

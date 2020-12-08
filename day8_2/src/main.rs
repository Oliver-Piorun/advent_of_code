use std::{collections::HashSet, error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut swapped_instruction_indices = HashSet::new();
    let mut swapped_instruction = false;

    let mut executed_instruction_indices = HashSet::new();
    let mut instruction_index = 0;
    let mut accumulator = 0;

    loop {
        if instruction_index >= lines.len() as i32 {
            println!("Finished!");
            break;
        } else if instruction_index < 0 {
            println!("Invalid instruction index!");
            break;
        }

        if !executed_instruction_indices.contains(&instruction_index) {
            executed_instruction_indices.insert(instruction_index);
        } else if swapped_instruction {
            println!("Swapped instruction still resulted in an infinite loop!");

            // Reset everything besides the swapped instruction indices and perform another attempt
            swapped_instruction = false;

            executed_instruction_indices.clear();
            instruction_index = 0;
            accumulator = 0;
            continue;
        } else {
            println!("Swapping all instructions still resulted in an infinite loop!");
            break;
        }

        let instruction = &lines[instruction_index as usize];

        let split: Vec<&str> = instruction.split(' ').collect();
        let mut operation = split[0];

        // Swap instruction if
        // - no instruction has been swapped for this attempt yet
        // - the operation is swappable
        // - the instruction has not been swapped yet
        if !swapped_instruction
            && (operation == "jmp" || operation == "nop")
            && !swapped_instruction_indices.contains(&instruction_index)
        {
            println!(
                "Swapping instruction (instruction index: {}) (swapped instructions: {})",
                instruction_index,
                swapped_instruction_indices.len()
            );

            operation = match operation {
                "jmp" => "nop",
                "nop" => "jmp",
                _ => operation,
            };

            swapped_instruction_indices.insert(instruction_index);
            swapped_instruction = true;
        }

        let argument = split[1].parse::<i32>()?;

        match operation {
            "acc" => {
                accumulator += argument;
                instruction_index += 1;
            }
            "jmp" => instruction_index += argument,
            "nop" => instruction_index += 1,
            _ => panic!("Unhandled operation! ({})", operation),
        }
    }

    println!("{}", accumulator);

    Ok(())
}

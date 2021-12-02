use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

enum CommandType {
    Forward,
    Down,
    Up,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(CommandType::Forward),
            "down" => Ok(CommandType::Down),
            "up" => Ok(CommandType::Up),
            _ => Err(()),
        }
    }
}

struct Command {
    pub r#type: CommandType,
    pub value: i32,
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input")?;

    // Read all lines and parse them into commands
    let commands: Vec<_> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split(' ')
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .map(|command| Command {
            r#type: CommandType::from_str(command[0].as_str()).unwrap(),
            value: command[1].parse::<i32>().unwrap(),
        })
        .collect();

    part1(&commands);
    part2(&commands);

    Ok(())
}

fn part1(commands: &[Command]) {
    let mut horizontal_position = 0;
    let mut depth = 0;

    // Take a look at each command
    for command in commands {
        match command.r#type {
            CommandType::Forward => horizontal_position += command.value, // Increase horizontal position
            CommandType::Down => depth += command.value,                  // Increase depth
            CommandType::Up => depth -= command.value,                    // Decrease depth
        }
    }

    println!("part1: {}", horizontal_position * depth);
}

fn part2(commands: &[Command]) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    // Take a look at each command
    for command in commands {
        match command.r#type {
            CommandType::Forward => {
                // Increase horizontal position
                horizontal_position += command.value;

                // Increase depth
                depth += aim * command.value;
            }
            CommandType::Down => aim += command.value, // Increase aim
            CommandType::Up => aim -= command.value,   // Decrease aim
        }
    }

    println!("part2: {}", horizontal_position * depth);
}

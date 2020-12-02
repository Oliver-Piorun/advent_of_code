use std::io::{self, BufReader};
use std::{fs::File, io::BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let lines: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.map(|line_str| line_str.parse::<i32>().unwrap())
                .unwrap()
        })
        .collect();

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if lines[i] + lines[j] == 2020 {
                println!("{} + {} = 2020", lines[i], lines[j]);
                println!("{} * {} = {}", lines[i], lines[j], lines[i] * lines[j]);
            }
        }
    }

    Ok(())
}

use std::io::{self, BufReader};
use std::{fs::File, io::BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let lines: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            for k in j + 1..lines.len() {
                if lines[i] + lines[j] + lines[k] == 2020 {
                    println!("{} + {} + {} = 2020", lines[i], lines[j], lines[k]);
                    println!(
                        "{} * {} * {} = {}",
                        lines[i],
                        lines[j],
                        lines[k],
                        lines[i] * lines[j] * lines[k]
                    );
                }
            }
        }
    }

    Ok(())
}

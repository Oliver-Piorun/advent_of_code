use std::io::{self, BufReader};
use std::{fs::File, io::BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let symbols: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut num_trees = 0;
    let mut column_index: usize = 0;

    for (row_index, _) in symbols.iter().enumerate().skip(1) {
        column_index += 3;

        let max_column_index = symbols[row_index].len() - 1;
        let column_index_diff = max_column_index as i32 - column_index as i32;

        // Check if we have to wrap
        if column_index_diff < 0 {
            column_index = (-column_index_diff - 1) as usize;
        }

        let symbol = symbols[row_index][column_index];

        if symbol == '#' {
            println!(
                "{} {} {}",
                row_index, column_index, symbols[row_index][column_index]
            );
            num_trees += 1;
        }
    }

    println!("{}", num_trees);

    Ok(())
}

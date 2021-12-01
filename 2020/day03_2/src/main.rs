use std::io::{self, BufReader};
use std::{fs::File, io::BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let symbols: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut num_multiplied_trees: i64 = 0;

    // Iterate over each slope
    for (right, down) in slopes {
        let mut num_trees = 0;
        let mut column_index: usize = 0;

        for (row_index, _) in symbols.iter().enumerate().skip(down).step_by(down) {
            column_index += right;

            let max_column_index = symbols[row_index].len() - 1;
            let column_index_diff = max_column_index as i32 - column_index as i32;

            // Check if we have to wrap
            if column_index_diff < 0 {
                column_index = (-column_index_diff - 1) as usize;
            }

            let symbol = symbols[row_index][column_index];

            if symbol == '#' {
                num_trees += 1;
            }
        }

        println!("{right} {down} {num_trees}");

        if num_multiplied_trees == 0 {
            num_multiplied_trees = num_trees;
        } else {
            num_multiplied_trees *= num_trees;
        }
    }

    println!("{num_multiplied_trees}");

    Ok(())
}

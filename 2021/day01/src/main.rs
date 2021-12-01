use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input")?;

    // Read all lines and parse them into numbers
    let depths: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    part1(&depths);
    part2(&depths);

    Ok(())
}

fn part1(depths: &[i32]) {
    let mut num_increases = 0;

    // Take a look at two depths at a time
    for window in depths.windows(2) {
        let first_depth = window[0];
        let second_depth = window[1];

        if second_depth > first_depth {
            // The depth increased because the second depth is greater than the first one
            num_increases += 1;
        }
    }

    println!("part1: {num_increases}");
}

fn part2(depths: &[i32]) {
    let mut num_increases = 0;

    // Take a look at four depths at a time
    for window in depths.windows(4) {
        let first_depth = window[0];
        let second_depth = window[1];
        let third_depth = window[2];
        let fourth_depth = window[3];

        // Calculate the sums of the two depth triplets
        let first_sum = first_depth + second_depth + third_depth;
        let second_sum = second_depth + third_depth + fourth_depth;

        if second_sum > first_sum {
            // The depth increased because the second sum of depth triplets is greater than the first one
            num_increases += 1;
        }
    }

    println!("part2: {num_increases}");
}

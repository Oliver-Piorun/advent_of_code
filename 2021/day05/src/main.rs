#![feature(int_abs_diff)]
use std::{
    cmp::{max, Ordering},
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

struct VentLine {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input")?;

    // Get all vent lines
    let vent_lines = BufReader::new(file)
        .lines()
        .map(|line| {
            // Replace " -> " with "," so that we can get the coordinates more easily
            let sanitized_line = line.unwrap().replace(" -> ", ",");

            // Split at "," and parse the coordinates into numbers
            let mut coordinates = sanitized_line
                .split(',')
                .map(|coordinate_str| coordinate_str.parse::<u16>().unwrap());

            VentLine {
                x1: coordinates.next().unwrap(),
                y1: coordinates.next().unwrap(),
                x2: coordinates.next().unwrap(),
                y2: coordinates.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    part1(&vent_lines);
    part2(&vent_lines);

    Ok(())
}

fn part1(vent_lines: &[VentLine]) {
    let horizontal_and_vertical_vent_lines = vent_lines
        .iter()
        .filter(|vent_line| vent_line.x1 == vent_line.x2 || vent_line.y1 == vent_line.y2)
        .collect::<Vec<_>>();

    println!(
        "part1: {}",
        get_num_two_or_more_occurrences(&horizontal_and_vertical_vent_lines)
    );
}

fn part2(vent_lines: &[VentLine]) {
    let all_vent_lines = vent_lines.iter().collect::<Vec<_>>();

    println!(
        "part2: {}",
        get_num_two_or_more_occurrences(&all_vent_lines)
    );
}

fn get_num_two_or_more_occurrences(vent_lines: &[&VentLine]) -> u16 {
    let mut covered_points = HashMap::<(u16, u16), u16>::new();

    for vent_line in vent_lines {
        let x1 = vent_line.x1;
        let y1 = vent_line.y1;
        let x2 = vent_line.x2;
        let y2 = vent_line.y2;

        let x_step = match x1.cmp(&x2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let y_step = match y1.cmp(&y2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        let x_abs = x1.abs_diff(x2);
        let y_abs = y1.abs_diff(y2);

        let num_steps = max(x_abs, y_abs) + 1;
        let mut x = x1;
        let mut y = y1;

        // Perform the necessary number of steps and follow the points on the line
        for _ in 1..=num_steps {
            // Increase the number of occurrences for the current point
            *covered_points.entry((x, y)).or_insert(0) += 1;

            // Move to the next point
            x = (x as i32 + x_step) as u16;
            y = (y as i32 + y_step) as u16;
        }
    }

    // Return the number of points which were covered at least twice
    covered_points.values().fold(
        0,
        |acc, occurrences| if *occurrences >= 2 { acc + 1 } else { acc },
    )
}

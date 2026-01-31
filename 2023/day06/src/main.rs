// https://adventofcode.com/2023/day/6
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut time_records = Vec::new();

    // Skip "Time:     "
    input_index += 10;

    loop {
        skip_potential_whitespace(input, &mut input_index);

        let time_record = read_value(input, &mut input_index);
        time_records.push(time_record);

        if input[input_index] == b'\n' {
            break;
        }
    }

    // Skip "\nDistance: "
    input_index += 11;

    let mut distance_records = Vec::new();

    loop {
        skip_potential_whitespace(input, &mut input_index);

        let distance_record = read_value(input, &mut input_index);
        distance_records.push(distance_record);

        if input_index == input.len() {
            break;
        }
    }

    // We assume that there is at least one possibility
    let mut possibilities_product = 1;

    for (index, &time_record) in time_records.iter().enumerate() {
        let distance_record = distance_records[index];

        possibilities_product *= get_possibilities(time_record as u64, distance_record as u64);
    }

    possibilities_product
}

#[inline(always)]
fn part2() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    // Skip "Time:     "
    input_index += 10;

    let time_record = read_value_while_skipping_potential_whitespace(input, &mut input_index);

    // Skip "\nDistance: "
    input_index += 11;

    let distance_record = read_value_while_skipping_potential_whitespace(input, &mut input_index);

    get_possibilities(time_record, distance_record)
}

#[inline(always)]
fn skip_potential_whitespace(input: &[u8], input_index: &mut usize) {
    while input[*input_index] == b' ' {
        *input_index += 1;
    }
}

#[inline(always)]
fn read_value(input: &[u8], input_index: &mut usize) -> u16 {
    let mut value = 0;

    while *input_index < input.len() && input[*input_index] != b' ' && input[*input_index] != b'\n'
    {
        value = value * 10 + (input[*input_index] & 0xF) as u16;

        *input_index += 1;
    }

    value
}

#[inline(always)]
fn read_value_while_skipping_potential_whitespace(input: &[u8], input_index: &mut usize) -> u64 {
    let mut value = 0;

    while *input_index < input.len() && input[*input_index] != b'\n' {
        if input[*input_index] == b' ' {
            *input_index += 1;
            continue;
        }

        value = value * 10 + (input[*input_index] & 0xF) as u64;

        *input_index += 1;
    }

    value
}

#[inline(always)]
fn get_possibilities(time_record: u64, distance_record: u64) -> u32 {
    // Example:
    // time record: 12, distance record: 30
    // power: 1, 11 seconds left -> 11 *  1 = 11
    // power: 2, 10 seconds left -> 10 *  2 = 20
    // power: 3,  9 seconds left ->  9 *  3 = 27
    // power: 4,  8 seconds left ->  8 *  4 = 32 <- Start of the winning streak
    // power: 5,  7 seconds left ->  7 *  5 = 35
    // power: 6,  6 seconds left ->  6 *  6 = 36
    // power: 7,  5 seconds left ->  5 *  7 = 35
    // power: 8,  4 seconds left ->  4 *  8 = 32 <- End of the winning streak
    // power: 9,  3 seconds left ->  3 *  9 = 27
    // power: 10, 2 seconds left ->  2 * 10 = 20
    // power: 11, 1 second left  ->  1 * 11 = 11
    //
    // General quadratic function: y(x) = a*x^2 + b*x + c
    // Our quadratic function: y(power) = (time record - power) * power
    //
    // Find the real roots:
    // -power^2 + time record * power = distance record
    // -power^2 + time record * power - distance record = 0
    // a = -1, b = time record, c = -distance record
    // x1,2 = (-b +- sqrt(b^2 - 4 * a * c)) / (2 * a)
    let a = -1.0;
    let b = time_record as f64;
    let c = -(distance_record as f64);

    let sqrt = (b * b - 4.0 * a * c).sqrt();

    let first_root = (-b + sqrt) / (2.0 * a);
    let second_root = (-b - sqrt) / (2.0 * a);

    (second_root.floor() - first_root.ceil()) as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 503424);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 32607562);
    }

    #[test]
    fn test_get_possibilities() {
        assert_eq!(get_possibilities(12, 30), 5);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| black_box(part1()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| black_box(part2()));
    }
}

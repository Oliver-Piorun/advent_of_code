// https://adventofcode.com/2023/day/1
#![feature(test)]
extern crate test;

use std::cmp::{max, min};

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut calibration_value_sum = 0;

    loop {
        let mut line_index = input_index;

        let mut first_digit_char = -1;
        let mut last_digit_char = 0;

        while line_index < input.len() && input[line_index] != b'\n' {
            if input[line_index] >= b'1' && input[line_index] <= b'9' {
                if first_digit_char == -1 {
                    first_digit_char = input[line_index] as i8;
                }

                last_digit_char = input[line_index];
            }

            line_index += 1;
        }

        calibration_value_sum +=
            ((first_digit_char as u8 - b'0') * 10 + last_digit_char - b'0') as u32;

        if line_index == input.len() {
            break;
        }

        input_index = line_index + 1;
    }

    calibration_value_sum
}

#[inline(always)]
fn part2() -> u32 {
    const DIGITS_AS_STR: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut calibration_value_sum = 0;

    loop {
        let mut line_index = input_index;

        let mut first_digit = 0;
        let mut last_digit = 0;

        while line_index < input.len() && input[line_index] != b'\n' {
            if input[line_index] >= b'1' && input[line_index] <= b'9' {
                first_digit = input[line_index] - b'0';
                break;
            } else {
                let to_line_index = min(line_index + 4, input.len() - 1);
                let substring = std::str::from_utf8(&input[line_index..=to_line_index]).unwrap();

                let matched_digit_as_str = DIGITS_AS_STR
                    .iter()
                    .filter(|&&digit_as_str| substring.starts_with(digit_as_str))
                    .next();

                if let Some(matched_digit_as_str) = matched_digit_as_str {
                    first_digit = map_to_digit(matched_digit_as_str);
                    break;
                }
            }

            line_index += 1;
        }

        while line_index < input.len() && input[line_index] != b'\n' {
            line_index += 1;
        }

        let end_index = line_index;

        line_index -= 1;

        while input[line_index] != b'\n' {
            if input[line_index] >= b'1' && input[line_index] <= b'9' {
                last_digit = input[line_index] - b'0';
                break;
            } else {
                let from_line_index = max(0, line_index as isize - 4) as usize;
                let substring = std::str::from_utf8(&input[from_line_index..=line_index]).unwrap();

                let matched_digit_as_str = DIGITS_AS_STR
                    .iter()
                    .filter(|&&digit_as_str| substring.ends_with(digit_as_str))
                    .next();

                if let Some(matched_digit_as_str) = matched_digit_as_str {
                    last_digit = map_to_digit(matched_digit_as_str);
                    break;
                }
            }

            if line_index == 0 {
                break;
            }

            line_index -= 1;
        }

        calibration_value_sum += (first_digit * 10 + last_digit) as u32;

        line_index = end_index;

        if line_index == input.len() {
            break;
        }

        input_index = line_index + 1;
    }

    calibration_value_sum
}

#[inline(always)]
fn map_to_digit(digit_as_str: &str) -> u8 {
    match digit_as_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 54644);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 53348);
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

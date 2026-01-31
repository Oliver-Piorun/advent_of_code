// https://adventofcode.com/2023/day/1
#![feature(test)]
extern crate test;

use memchr::memchr;
use std::cmp::{max, min};

fn main() {
    part1();
    part2();
}

fn part1() -> u32 {
    get_calibration_value_sum(false)
}

fn part2() -> u32 {
    get_calibration_value_sum(true)
}

#[inline(always)]
fn get_calibration_value_sum(consider_spelled_out_digits: bool) -> u32 {
    const SPELLED_OUT_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut calibration_value_sum = 0;

    loop {
        let mut line_index = input_index;

        let mut first_digit_char = 0;
        let last_digit_char;

        let line_break_index = memchr(b'\n', &input[line_index..]);

        let end_index;

        if let Some(line_break_index) = line_break_index {
            end_index = line_index + line_break_index;
        } else {
            end_index = input.len();
        }

        while line_index < end_index {
            if input[line_index] >= b'1' && input[line_index] <= b'9' {
                first_digit_char = input[line_index];
                break;
            } else if consider_spelled_out_digits {
                // The maximum number of letters for a spelled out digit is 5
                let to_line_index = min(line_index + 4, input.len() - 1);
                let substring = std::str::from_utf8(&input[line_index..=to_line_index]).unwrap();

                let matched_spelled_out_digit = SPELLED_OUT_DIGITS
                    .iter()
                    .find(|&&spelled_out_digit| substring.starts_with(spelled_out_digit));

                if let Some(matched_spelled_out_digit) = matched_spelled_out_digit {
                    first_digit_char = map_to_digit_char(matched_spelled_out_digit);
                    break;
                }
            }

            line_index += 1;
        }

        line_index = end_index - 1;

        loop {
            if input[line_index] >= b'1' && input[line_index] <= b'9' {
                last_digit_char = input[line_index];
                break;
            } else if consider_spelled_out_digits {
                // The maximum number of letters for a spelled out digit is 5
                let from_line_index = max(0, line_index as isize - 4) as usize;
                let substring = std::str::from_utf8(&input[from_line_index..=line_index]).unwrap();

                let matched_spelled_out_digit = SPELLED_OUT_DIGITS
                    .iter()
                    .find(|&&spelled_out_digit| substring.ends_with(spelled_out_digit));

                if let Some(matched_spelled_out_digit) = matched_spelled_out_digit {
                    last_digit_char = map_to_digit_char(matched_spelled_out_digit);
                    break;
                }
            }

            line_index -= 1;
        }

        calibration_value_sum += ((first_digit_char - b'0') * 10 + last_digit_char - b'0') as u32;

        line_index = end_index;

        if line_index == input.len() {
            break;
        }

        input_index = line_index + 1;
    }

    calibration_value_sum
}

#[inline(always)]
fn map_to_digit_char(spelled_out_digit: &str) -> u8 {
    match spelled_out_digit {
        "one" => b'1',
        "two" => b'2',
        "three" => b'3',
        "four" => b'4',
        "five" => b'5',
        "six" => b'6',
        "seven" => b'7',
        "eight" => b'8',
        "nine" => b'9',
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

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

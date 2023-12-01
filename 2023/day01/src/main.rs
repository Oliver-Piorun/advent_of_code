// https://adventofcode.com/2023/day/1
#![feature(test)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let file = File::open("input").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut calibration_value_sum = 0;

    for line in lines {
        let mut vec = Vec::new();
        for i in line.chars() {
            if i.is_ascii_digit() {
                vec.push(i);
            }
        }

        let calibration_value = (vec.first().unwrap().to_string()
            + &vec.last().unwrap().to_string())
            .as_str()
            .parse::<i32>()
            .unwrap();
        calibration_value_sum += calibration_value;
    }

    calibration_value_sum
}

#[inline(always)]
fn part2() -> u32 {
    let file = File::open("input").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut calibration_value_sum = 0;

    for line in lines {
        let first_digit = get_first_digit(&line);
        let last_digit = get_last_digit(&line);
        let calibration_value = first_digit * 10 + last_digit;

        calibration_value_sum += calibration_value as u32;
    }

    calibration_value_sum
}

fn get_first_digit(line: &str) -> u8 {
    let indices = [
        line.find("one").unwrap_or(usize::MAX),
        line.find("two").unwrap_or(usize::MAX),
        line.find("three").unwrap_or(usize::MAX),
        line.find("four").unwrap_or(usize::MAX),
        line.find("five").unwrap_or(usize::MAX),
        line.find("six").unwrap_or(usize::MAX),
        line.find("seven").unwrap_or(usize::MAX),
        line.find("eight").unwrap_or(usize::MAX),
        line.find("nine").unwrap_or(usize::MAX),
    ];

    let min_index = *indices.iter().min().unwrap();
    let mut index = usize::MAX;
    let mut digit = 0;

    if min_index != usize::MAX {
        index = min_index;
        digit = (indices
            .iter()
            .position(|index| *index == min_index)
            .unwrap()
            + 1) as u8;
    }

    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() && i <= index {
            digit = char.to_string().parse::<u8>().unwrap();
            break;
        }
    }

    digit
}

fn get_last_digit(line: &str) -> u8 {
    let indices = [
        line.rfind("one").map(|index| index as isize).unwrap_or(-1),
        line.rfind("two").map(|index| index as isize).unwrap_or(-1),
        line.rfind("three")
            .map(|index| index as isize)
            .unwrap_or(-1),
        line.rfind("four").map(|index| index as isize).unwrap_or(-1),
        line.rfind("five").map(|index| index as isize).unwrap_or(-1),
        line.rfind("six").map(|index| index as isize).unwrap_or(-1),
        line.rfind("seven")
            .map(|index| index as isize)
            .unwrap_or(-1),
        line.rfind("eight")
            .map(|index| index as isize)
            .unwrap_or(-1),
        line.rfind("nine").map(|index| index as isize).unwrap_or(-1),
    ];

    let max_index = *indices.iter().max().unwrap();
    let mut index = -1;
    let mut digit = 0;

    if max_index != -1 {
        index = max_index;
        digit = (indices
            .iter()
            .position(|index| *index == max_index)
            .unwrap()
            + 1) as u8;
    }

    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() && i as isize >= index {
            index = i as isize;
            digit = char.to_string().parse::<u8>().unwrap();
        }
    }

    digit
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

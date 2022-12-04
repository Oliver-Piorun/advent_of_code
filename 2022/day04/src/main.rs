// https://adventofcode.com/2022/day/4
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let input = include_str!("../input");
    let mut total_overlaps = 0;

    for line in input.lines() {
        let (first_section_from, first_section_to, second_section_from, second_section_to) =
            get_section_ids(line);

        if (first_section_from <= second_section_from && first_section_to >= second_section_to)
            || (second_section_from <= first_section_from && second_section_to >= first_section_to)
        {
            // One section is fully containing the other
            total_overlaps += 1;
        }
    }

    println!("part1: {total_overlaps}");

    total_overlaps
}

fn part2() -> i32 {
    let input = include_str!("../input");
    let mut total_overlaps = 0;

    for line in input.lines() {
        let (first_section_from, first_section_to, second_section_from, second_section_to) =
            get_section_ids(line);

        if first_section_from <= second_section_to && second_section_from <= first_section_to {
            // One section is overlapping the other
            total_overlaps += 1;
        }
    }

    println!("part2: {total_overlaps}");

    total_overlaps
}

fn get_section_ids(str: &str) -> (i32, i32, i32, i32) {
    let bytes = str.as_bytes();
    let comma_index = memchr::memchr(b',', bytes).unwrap();

    let mut section = &bytes[..comma_index];
    let mut dash_index = memchr::memchr(b'-', section).unwrap();
    let first_section_from = atoi::atoi::<i32>(&section[..dash_index]).unwrap();
    let first_section_to = atoi::atoi::<i32>(&section[dash_index + 1..]).unwrap();

    section = &bytes[comma_index + 1..];
    dash_index = memchr::memchr(b'-', section).unwrap();
    let second_section_from = atoi::atoi::<i32>(&section[..dash_index]).unwrap();
    let second_section_to = atoi::atoi::<i32>(&section[dash_index + 1..]).unwrap();

    (
        first_section_from,
        first_section_to,
        second_section_from,
        second_section_to,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 567);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 907);
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

// https://adventofcode.com/2022/day/4
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let input = include_bytes!("../input");
    let mut index = 0;
    let mut total_overlaps = 0;

    while index < input.len() {
        let first_section_from = get_section(input, &mut index, '-');
        let first_section_to = get_section(input, &mut index, ',');
        let second_section_from = get_section(input, &mut index, '-');
        let second_section_to = get_section(input, &mut index, '\n');

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
    let input = include_bytes!("../input");
    let mut index = 0;
    let mut total_overlaps = 0;

    while index < input.len() {
        let first_section_from = get_section(input, &mut index, '-');
        let first_section_to = get_section(input, &mut index, ',');
        let second_section_from = get_section(input, &mut index, '-');
        let second_section_to = get_section(input, &mut index, '\n');

        if first_section_from <= second_section_to && second_section_from <= first_section_to {
            // One section is overlapping the other
            total_overlaps += 1;
        }
    }

    println!("part2: {total_overlaps}");

    total_overlaps
}

#[inline(always)]
fn get_section(input: &[u8], index: &mut usize, delimiter: char) -> u8 {
    let mut section = *input.get(*index).unwrap() - b'0';
    *index += 1;

    if input.get(*index).unwrap() != &(delimiter as u8) {
        section = section * 10 + input.get(*index).unwrap() - b'0';
        *index += 2;
    } else {
        *index += 1;
    }

    section
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

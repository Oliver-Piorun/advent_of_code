// https://adventofcode.com/2023/day/6
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_str!("../input");

    let mut lines = input.lines();

    let time_records_str = lines.next().unwrap();
    let mut split = time_records_str.split(':');
    split.next();
    let second = split.next().unwrap().trim();
    let time_records = second
        .split_whitespace()
        .map(|time_record_str| time_record_str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distance_records_str = lines.next().unwrap();
    let mut split = distance_records_str.split(':');
    split.next();
    let second = split.next().unwrap().trim();
    let distance_records = second
        .split_whitespace()
        .map(|distance_record_str| distance_record_str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut possibilities_product = 0;

    for (i, time_record) in time_records.iter().enumerate() {
        let distance_record = distance_records[i];

        let possibilities = get_possibilities(*time_record, distance_record);

        if possibilities > 0 {
            if possibilities_product == 0 {
                possibilities_product = 1;
            }

            possibilities_product *= possibilities;
        }
    }

    possibilities_product
}

#[inline(always)]
fn part2() -> u32 {
    let input = include_str!("../input");

    let mut lines = input.lines();

    let time_records_str = lines.next().unwrap();
    let mut split = time_records_str.split(':');
    split.next();
    let second = split.next().unwrap().trim();
    let time_record = second
        .split_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()
        .unwrap();

    let distance_records_str = lines.next().unwrap();
    let mut split = distance_records_str.split(':');
    split.next();
    let second = split.next().unwrap().trim();
    let distance_record = second
        .split_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse::<u64>()
        .unwrap();

    get_possibilities(time_record, distance_record)
}

#[inline(always)]
fn get_possibilities(time_record: u64, distance_record: u64) -> u32 {
    let mut possibilities = 0;
    let mut button_press_time = 1;

    loop {
        let charge = button_press_time;
        let time = button_press_time;

        if time >= time_record {
            return possibilities;
        }

        let time_left = time_record - time;
        let distance = time_left * charge;

        if distance > distance_record {
            possibilities += 1;
        }

        button_press_time += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 503424);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 32607562);
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

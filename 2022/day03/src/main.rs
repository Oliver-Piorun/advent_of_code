// https://adventofcode.com/2022/day/3
#![feature(test)]
#![feature(iter_array_chunks)]
extern crate test;
use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let input = include_str!("../input");
    let mut sum_of_priorities = 0;

    for line in input.lines() {
        let (first_comp, second_comp) = line.split_at(line.len() / 2);
        let first_comp_set: HashSet<char> = HashSet::from_iter(first_comp.chars());
        let second_comp_set = HashSet::from_iter(second_comp.chars());

        let mut intersection = first_comp_set.intersection(&second_comp_set);
        let item_type = intersection.next().unwrap();

        sum_of_priorities += to_priority(*item_type);
    }

    println!("part1: {sum_of_priorities}");

    sum_of_priorities
}

fn part2() -> i32 {
    let input = include_str!("../input");
    let mut sum_of_priorities = 0;

    for [first_line, second_line, third_line] in input.lines().array_chunks() {
        let first_comp_set: HashSet<char> = HashSet::from_iter(first_line.chars());
        let second_comp_set = HashSet::from_iter(second_line.chars());
        let third_comp_set: HashSet<char> = HashSet::from_iter(third_line.chars());

        let first_intersection = first_comp_set.intersection(&second_comp_set);
        let first_intersection_set =
            HashSet::from_iter(first_intersection.map(|item_type| item_type.to_owned()));

        let mut second_intersection = first_intersection_set.intersection(&third_comp_set);
        let item_type = second_intersection.next().unwrap();

        sum_of_priorities += to_priority(*item_type);
    }

    println!("part2: {sum_of_priorities}");

    sum_of_priorities
}

fn to_priority(item_type: char) -> i32 {
    if ('a'..='z').contains(&item_type) {
        // Start with a = 1
        item_type as i32 - 'a' as i32 + 1
    } else {
        // Start with A = 27
        item_type as i32 - 'A' as i32 + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 7737);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2697);
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

// https://adventofcode.com/2022/day/6
#![feature(test)]

use std::collections::HashSet;
extern crate test;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let input = include_str!("../input");
    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        let set: HashSet<&char> = HashSet::from_iter(&chars[i..i + 4]);

        if set.len() == 4 {
            return i as i32 + 4;
        }
    }

    panic!("We should never land here!");
}

fn part2() -> i32 {
    let input = include_str!("../input");
    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        let set: HashSet<&char> = HashSet::from_iter(&chars[i..i + 14]);

        if set.len() == 14 {
            return i as i32 + 14;
        }
    }

    panic!("We should never land here!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1909);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3380);
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

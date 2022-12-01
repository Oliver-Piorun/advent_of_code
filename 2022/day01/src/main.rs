#![feature(test)]
extern crate test;

use std::io;

fn main() -> io::Result<()> {
    part1();
    part2();

    Ok(())
}

fn part1() -> i32 {
    let calorie_sums = get_calorie_sums();
    let max_calories = calorie_sums.iter().max().unwrap();
    println!("part1: {max_calories}");

    *max_calories
}

fn part2() -> i32 {
    let mut calorie_sums = get_calorie_sums();
    calorie_sums.sort_unstable_by(|a, b| b.cmp(a));
    let calories = calorie_sums.iter().take(3).sum::<i32>();
    println!("part2: {calories}");

    calories
}

fn get_calorie_sums() -> Vec<i32> {
    let input = include_str!("../input");
    input
        .split("\n\n")
        .map(|calorie_group| {
            calorie_group
                .split('\n')
                .map(|calorie_entry| calorie_entry.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 70720);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 207148);
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

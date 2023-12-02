// https://adventofcode.com/2023/day/1
#![feature(test)]
extern crate test;

use regex::Regex;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_str!("../input");

    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();

    let mut possible_game_id_sum = 0;

    for line in input.lines() {
        let mut split = line.split(": ");
        let first = split.next().unwrap();
        let last = split.next().unwrap();

        let mut game_split = first.split(' ');
        game_split.next();
        let game_id = game_split.next().unwrap().parse::<i32>().unwrap();

        let sets = last.split("; ").collect::<Vec<_>>();

        let mut is_possible_game_id = true;

        for set in sets {
            if let Some(caps) = red_regex.captures(set) {
                let count = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

                if count > 12 {
                    is_possible_game_id = false;
                }
            }

            if let Some(caps) = green_regex.captures(set) {
                let count = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

                if count > 13 {
                    is_possible_game_id = false;
                }
            }

            if let Some(caps) = blue_regex.captures(set) {
                let count = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();

                if count > 14 {
                    is_possible_game_id = false;
                }
            }
        }

        if is_possible_game_id {
            possible_game_id_sum += game_id;
        }
    }

    possible_game_id_sum
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_str!("../input");

    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();

    let mut power_sum = 0;

    for line in input.lines() {
        let mut split = line.split(": ");
        let first = split.next().unwrap();
        let last = split.next().unwrap();

        let mut game_split = first.split(' ');
        game_split.next();

        let sets = last.split("; ").collect::<Vec<_>>();

        let mut red_max = -1;
        let mut green_max = -1;
        let mut blue_max = -1;

        for set in sets {
            if let Some(captures) = red_regex.captures(set) {
                let count = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();

                if count > red_max {
                    red_max = count;
                }
            }

            if let Some(captures) = green_regex.captures(set) {
                let count = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();

                if count > green_max {
                    green_max = count;
                }
            }

            if let Some(captures) = blue_regex.captures(set) {
                let count = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();

                if count > blue_max {
                    blue_max = count;
                }
            }
        }

        power_sum += red_max * green_max * blue_max;
    }

    println!("{power_sum}");

    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2164);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 69929);
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

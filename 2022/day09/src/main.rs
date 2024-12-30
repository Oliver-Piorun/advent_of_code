// https://adventofcode.com/2022/day/9
#![allow(clippy::comparison_chain)]
#![feature(test)]
extern crate test;

use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_str!("../input");

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(0);

    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap();
        let count = split.next().unwrap().parse::<u8>().unwrap();

        for _ in 0..count {
            match dir {
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                _ => panic!("We should never land here!"),
            };

            if i16::abs(head.0 - tail.0) == 2 || i16::abs(head.1 - tail.1) == 2 {
                if head.0 < tail.0 {
                    tail.0 -= 1;
                } else if head.0 > tail.0 {
                    tail.0 += 1;
                }

                if head.1 < tail.1 {
                    tail.1 -= 1;
                } else if head.1 > tail.1 {
                    tail.1 += 1;
                }

                visited.insert(((tail.0 as i32) << 16) | tail.1 as i32 & 0xFFFF);
            }
        }
    }

    visited.len() as i32
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_str!("../input");

    let mut rope = [(0, 0); 10];

    let mut visited = HashSet::new();
    visited.insert(0);

    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap();
        let count = split.next().unwrap().parse::<u8>().unwrap();

        for _ in 0..count {
            for i in 0..rope.len() - 1 {
                let (first, second) = rope.split_at_mut(i + 1);
                let head = first.last_mut().unwrap();
                let tail = second.first_mut().unwrap();

                if i == 0 {
                    match dir {
                        "L" => head.0 -= 1,
                        "R" => head.0 += 1,
                        "U" => head.1 -= 1,
                        "D" => head.1 += 1,
                        _ => panic!("We should never land here!"),
                    };
                }

                if i32::abs(head.0 - tail.0) == 2 || i32::abs(head.1 - tail.1) == 2 {
                    if head.0 < tail.0 {
                        tail.0 -= 1;
                    } else if head.0 > tail.0 {
                        tail.0 += 1;
                    }

                    if head.1 < tail.1 {
                        tail.1 -= 1;
                    } else if head.1 > tail.1 {
                        tail.1 += 1;
                    }

                    if i == 8 {
                        visited.insert((tail.0 << 16) | tail.1 & 0xFFFF);
                    }
                }
            }
        }
    }

    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6269);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2557);
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

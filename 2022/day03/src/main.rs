// https://adventofcode.com/2022/day/3
#![feature(test)]
#![feature(iter_array_chunks)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_str!("../input");
    let mut sum_of_priorities = 0;

    'outer: for bag in input.lines() {
        let (first_comp, second_comp) = bag.split_at(bag.len() / 2);

        for i in first_comp.chars() {
            for j in second_comp.chars() {
                if i != j {
                    continue;
                }

                sum_of_priorities += to_priority(i);
                continue 'outer;
            }
        }
    }

    sum_of_priorities
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_str!("../input");
    let mut sum_of_priorities = 0;

    'outer: for [first_bag, second_bag, third_bag] in input.lines().array_chunks() {
        for i in first_bag.chars() {
            for j in second_bag.chars() {
                if i != j {
                    continue;
                }

                for k in third_bag.chars() {
                    if i != k {
                        continue;
                    }

                    sum_of_priorities += to_priority(i);
                    continue 'outer;
                }
            }
        }
    }

    sum_of_priorities
}

#[inline(always)]
fn to_priority(item_type: char) -> i32 {
    if item_type >= 'a' {
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

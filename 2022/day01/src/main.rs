// https://adventofcode.com/2022/day/1
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let calorie_sums = get_calorie_sums();
    let max_calories = calorie_sums.iter().max().unwrap();

    *max_calories
}

#[inline(always)]
fn part2() -> i32 {
    let mut calorie_sums = get_calorie_sums();
    calorie_sums.sort_unstable_by(|a, b| b.cmp(a));

    calorie_sums.iter().take(3).sum::<i32>()
}

#[inline(always)]
fn get_calorie_sums() -> Vec<i32> {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut calorie_sums = Vec::new();
    let mut calorie_sum = 0;
    let mut calories = 0;

    while input_index < input.len() {
        if input[input_index] != b'\n' {
            calories = calories * 10 + (input[input_index] - b'0') as i32;
        } else {
            calorie_sum += calories;
            calories = 0;

            if input[input_index + 1] == b'\n' {
                calorie_sums.push(calorie_sum);
                calorie_sum = 0;
                input_index += 1;
            }
        }

        input_index += 1;
    }

    calorie_sum += calories;
    calorie_sums.push(calorie_sum);

    calorie_sums
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

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

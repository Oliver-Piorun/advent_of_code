// https://adventofcode.com/2023/day/9
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_str!("../input");

    let mut extrapolated_value_sum = 0;

    for line in input.lines() {
        let values = line
            .split(' ')
            .map(|value_as_str| value_as_str.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut sequences = vec![values];

        let mut sequence_index = 0;

        loop {
            let mut new_sequence = Vec::new();
            let mut all_zeros = true;

            let sequence = sequences.get(sequence_index).unwrap();

            for (value_index, value) in sequence.iter().enumerate() {
                if value_index == sequence.len() - 1 {
                    break;
                }

                let difference = sequence.get(value_index + 1).unwrap() - value;

                if difference != 0 {
                    all_zeros = false;
                }

                new_sequence.push(difference);
            }

            sequences.push(new_sequence);

            if all_zeros {
                break;
            }

            sequence_index += 1;
        }

        let mut extrapolated_value = 0;

        for sequence in sequences.iter().rev().skip(1) {
            let last_value = sequence.last().unwrap();
            extrapolated_value += last_value;
        }

        extrapolated_value_sum += extrapolated_value;
    }

    extrapolated_value_sum
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_str!("../input");

    let mut extrapolated_value_sum = 0;

    for line in input.lines() {
        let values = line
            .split(' ')
            .map(|value_as_str| value_as_str.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut sequences = vec![values];

        let mut sequence_index = 0;

        loop {
            let mut new_sequence = Vec::new();
            let mut all_zeros = true;

            let sequence = sequences.get(sequence_index).unwrap();

            for (value_index, value) in sequence.iter().enumerate() {
                if value_index == sequence.len() - 1 {
                    break;
                }

                let diff = sequence.get(value_index + 1).unwrap() - value;

                if diff != 0 {
                    all_zeros = false;
                }

                new_sequence.push(diff);
            }

            sequences.push(new_sequence);

            if all_zeros {
                break;
            }

            sequence_index += 1;
        }

        let mut extrapolated_value = 0;

        for sequence in sequences.iter().rev().skip(1) {
            let first_value = sequence.first().unwrap();
            extrapolated_value = first_value - extrapolated_value;
        }

        extrapolated_value_sum += extrapolated_value;
    }

    extrapolated_value_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1898776583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1100);
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

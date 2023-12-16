// https://adventofcode.com/2023/day/9
#![feature(portable_simd)]
#![feature(test)]
extern crate test;

use std::simd::i32x32;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    get_extrapolated_value_sum(|sequence, sequence_length, extrapolated_value| {
        *extrapolated_value += sequence[*sequence_length - 1];
        *sequence_length += 1;
    })
}

#[inline(always)]
fn part2() -> i32 {
    get_extrapolated_value_sum(|sequence, _, extrapolated_value| {
        *extrapolated_value = sequence[0] - *extrapolated_value
    })
}

#[inline(always)]
fn get_extrapolated_value_sum<F>(calculate_extrapolated_value_fn: F) -> i32
where
    F: Fn(
        &[i32; 32], // sequence
        &mut usize, // sequence_length
        &mut i32,   // extrapolated_value
    ),
{
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut extrapolated_value_sum = 0;

    loop {
        let mut values = [0; 32];
        let mut value_index = 0;

        loop {
            let value = read_value(input, &mut input_index);
            values[value_index] = value;
            value_index += 1;

            if input_index == input.len() || input[input_index] == b'\n' {
                break;
            }

            // Skip " "
            input_index += 1;
        }

        let mut sequence_length = value_index;
        let mut sequences = vec![values];
        let mut sequence_index = 0;

        loop {
            let sequence = &sequences[sequence_index];

            // Rotate left to be able to subtract properly. We do not care about substraction results which are at
            // indices >= sequence_length. Those are not taken into account when checking for zeros or when calculating
            // the extrapolated values
            let minuend = i32x32::from_slice(sequence).rotate_lanes_left::<1>();
            let subtrahend = i32x32::from_slice(sequence);
            let difference = minuend - subtrahend;

            let new_sequence = difference.as_array();

            if new_sequence[0..sequence_length - 1]
                .iter()
                .all(|&value| value == 0)
            {
                break;
            }

            sequence_length -= 1;
            sequences.push(*new_sequence);
            sequence_index += 1;
        }

        let mut extrapolated_value = 0;

        for sequence in sequences.iter().rev() {
            calculate_extrapolated_value_fn(
                sequence,
                &mut sequence_length,
                &mut extrapolated_value,
            );
        }

        extrapolated_value_sum += extrapolated_value;

        if input_index == input.len() {
            break;
        }

        // Skip "\n"
        input_index += 1;
    }

    extrapolated_value_sum
}

#[inline(always)]
fn read_value(input: &[u8], input_index: &mut usize) -> i32 {
    let mut factor = 1;
    let mut value = 0;

    while *input_index < input.len() && input[*input_index] != b' ' && input[*input_index] != b'\n'
    {
        if input[*input_index] == b'-' {
            factor = -1;
        } else {
            value = value * 10 + (input[*input_index] & 0xF) as i32;
        }

        *input_index += 1;
    }

    factor * value
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

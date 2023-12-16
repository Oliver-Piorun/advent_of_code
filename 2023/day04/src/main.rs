// https://adventofcode.com/2023/day/4
#![feature(portable_simd)]
#![feature(test)]
extern crate test;

#[cfg(target_arch = "x86")]
use std::arch::x86::{__m128i, _mm_cmpeq_epi8, _mm_movemask_epi8, _mm_set1_epi8};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m128i, _mm_cmpeq_epi8, _mm_movemask_epi8, _mm_set1_epi8};
use std::simd::u8x16;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut points = 0;
    let mut their_card = u8x16::default();

    loop {
        // Skip "Card<whitespace><number>: "
        input_index += 10;

        // Iterate over their card
        for i in 0..10 {
            skip_potential_whitespace(input, &mut input_index);
            their_card[i] = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;
        }

        // Skip "| "
        input_index += 2;

        let mut num_winning_numbers = 0;

        // Iterate over our card
        for _ in 0..25 {
            skip_potential_whitespace(input, &mut input_index);
            let our_number = read_value(input, &mut input_index);

            let their_card = __m128i::from(their_card);

            let their_card_contains_our_number = unsafe {
                // Set all 16 elements to "our_number"
                let our_number = _mm_set1_epi8(our_number as i8);

                // Only the 80 most significant bits (10 bytes = 10 x 8 bits = 80 bits) are relevant.
                // The comparison result of the 48 least significant bits (6 bytes = 6 x 8 bits = 48 bits) will never
                // produce hits because those bits are all zeros and "our_number" cannot be zero
                let comparison_result = _mm_cmpeq_epi8(their_card, our_number);

                let mask = _mm_movemask_epi8(comparison_result);

                mask > 0
            };

            if their_card_contains_our_number {
                num_winning_numbers += 1;
            }
        }

        if num_winning_numbers > 0 {
            points += 2_u32.pow(num_winning_numbers - 1);
        }

        if input_index == input.len() {
            return points;
        }

        // Skip "\n"
        input_index += 1;
    }
}

#[inline(always)]
fn part2() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let line_length = input
        .iter()
        .position(|&character| character == b'\n')
        .unwrap();
    let num_cards = (input.len() / (line_length + 1)) + 1;
    let mut their_card = u8x16::default();
    let mut our_card_counts = vec![1u32; num_cards];

    for card_index in 0..num_cards {
        // Skip "Card<whitespace><number>: "
        input_index += 10;

        // Iterate over their card
        for i in 0..10 {
            skip_potential_whitespace(input, &mut input_index);
            their_card[i] = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;
        }

        // Skip "| "
        input_index += 2;

        let mut num_winning_numbers = 0;

        // Iterate over our card
        for _ in 0..25 {
            skip_potential_whitespace(input, &mut input_index);
            let our_number = read_value(input, &mut input_index);

            let their_card = __m128i::from(their_card);

            let their_card_contains_our_number = unsafe {
                // Set all 16 elements to "our_number"
                let our_number = _mm_set1_epi8(our_number as i8);

                // Only the 80 most significant bits (10 bytes = 10 x 8 bits = 80 bits) are relevant.
                // The comparison result of the 48 least significant bits (6 bytes = 6 x 8 bits = 48 bits) will never
                // produce hits because those bits are all zeros and "our_number" cannot be zero
                let comparison_result = _mm_cmpeq_epi8(their_card, our_number);

                let mask = _mm_movemask_epi8(comparison_result);

                mask > 0
            };

            if their_card_contains_our_number {
                num_winning_numbers += 1;
            }
        }

        if num_winning_numbers > 0 {
            for i in 0..num_winning_numbers {
                our_card_counts[card_index + 1 + i] += our_card_counts[card_index];
            }
        }

        // Skip "\n"
        input_index += 1;
    }

    our_card_counts.iter().sum()
}

#[inline(always)]
fn skip_potential_whitespace(input: &[u8], input_index: &mut usize) {
    while input[*input_index] == b' ' {
        *input_index += 1;
    }
}

#[inline(always)]
fn read_value(input: &[u8], input_index: &mut usize) -> u8 {
    let mut value = 0;

    // We only expect EOF, " ", "\n" or a digit
    // "\n" -> 0x12
    // " "  -> 0x20
    while *input_index < input.len() && input[*input_index] > b' ' {
        value *= 10;
        value += input[*input_index] - b'0';

        *input_index += 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 25183);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 5667240);
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

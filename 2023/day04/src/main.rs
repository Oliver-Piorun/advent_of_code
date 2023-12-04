// https://adventofcode.com/2023/day/4
#![feature(portable_simd)]
#![feature(test)]
extern crate test;

#[cfg(target_arch = "x86")]
use std::arch::x86::{__m128i, _mm_cmpeq_epi8, _mm_movemask_epi8, _mm_set1_epi8};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m128i, _mm_cmpeq_epi8, _mm_movemask_epi8, _mm_set1_epi8};
use std::collections::HashMap;
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

    let mut their_card_simd = u8x16::default();
    let mut num_winning_numbers = 0;

    loop {
        // Skip "Card<whitespace><number>: "
        input_index += 10;

        for i in 0..10 {
            skip_potential_whitespace(input, &mut input_index);
            their_card_simd[i] = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;
        }

        // Skip "| "
        input_index += 2;

        // Iterate over our card
        for _ in 0..25 {
            skip_potential_whitespace(input, &mut input_index);
            let our_number = read_value(input, &mut input_index);

            let their_card_simd = __m128i::from(their_card_simd);

            let their_card_contains_our_number = unsafe {
                // Set all 16 elements to "our_number"
                let our_number_simd = _mm_set1_epi8(our_number as i8);

                // Only the 80 most significant bits (10 bytes = 10 x 8 bits = 80 bits) are relevant.
                // The comparison result of the 48 least significant bits (6 bytes = 6 x 8 bits = 48 bits) will never
                // produce hits because those bits are all zeros and "our_number" cannot be zero
                let comparison_result = _mm_cmpeq_epi8(their_card_simd, our_number_simd);

                let mask = _mm_movemask_epi8(comparison_result);

                mask > 0
            };

            if their_card_contains_our_number {
                num_winning_numbers += 1;
            }
        }

        if num_winning_numbers > 0 {
            points += 2_u32.pow(num_winning_numbers - 1);
            num_winning_numbers = 0;
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
    let input = include_str!("../input");

    let mut their_cards = Vec::new();
    let mut our_numbers_vec = Vec::new();

    for line in input.lines() {
        let mut split = line.split(':');
        split.next();
        let second = split.next().unwrap().trim();
        let mut split2 = second.split('|');

        let their_card_str = split2.next().unwrap().trim().replace("  ", " ");
        let our_numbers_str = split2.next().unwrap().trim().replace("  ", " ");

        let their_card = their_card_str
            .split(' ')
            .map(|str| str.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        their_cards.push(their_card);

        let our_numbers = our_numbers_str
            .split(' ')
            .map(|str| str.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        our_numbers_vec.push(our_numbers);
    }

    let mut our_cards_map = HashMap::new();

    for (index, their_card) in their_cards.iter().enumerate() {
        let num_card = index + 1;
        let card_to_add = our_numbers_vec[num_card - 1].clone();

        let entry = our_cards_map.entry(num_card);

        match entry {
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(vec![card_to_add]);
            }
            std::collections::hash_map::Entry::Occupied(o) => {
                let our_cards = o.into_mut();
                our_cards.push(card_to_add);
            }
        }

        let mut copied_carts_to_add_map = HashMap::new();

        let our_cards = our_cards_map.get(&num_card).unwrap();

        for our_card in our_cards {
            let mut num_winning_numbers = 0;

            for our_number in our_card {
                if their_card.contains(our_number) {
                    num_winning_numbers += 1;
                }
            }

            if num_winning_numbers == 0 {
                break;
            }

            for i in 1..=num_winning_numbers {
                let num_our_following_card = num_card + i;

                if num_our_following_card > our_numbers_vec.len() {
                    break;
                }

                let card_to_copy = our_numbers_vec[num_our_following_card - 1].clone();

                let entry = copied_carts_to_add_map.entry(num_our_following_card);

                match entry {
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(vec![card_to_copy]);
                    }
                    std::collections::hash_map::Entry::Occupied(o) => {
                        let our_cards = o.into_mut();
                        our_cards.push(card_to_copy);
                    }
                }
            }
        }

        for (num_card, copied_cart_to_add) in copied_carts_to_add_map {
            let entry = our_cards_map.entry(num_card);

            match entry {
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(copied_cart_to_add);
                }
                std::collections::hash_map::Entry::Occupied(o) => {
                    let our_cards = o.into_mut();
                    our_cards.extend(copied_cart_to_add);
                }
            }
        }
    }

    our_cards_map
        .values()
        .map(|our_cards| our_cards.len() as u32)
        .sum()
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

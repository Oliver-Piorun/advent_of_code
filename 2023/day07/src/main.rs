// https://adventofcode.com/2023/day/7
#![feature(slice_group_by)]
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut hands = Vec::new();

    loop {
        let mut hand = [0u8; 5];

        for card in &mut hand {
            // Remap ASCII chars in a way that preserves the strengths of the camel cards while making comparing
            // them easier
            *card = match input[input_index] {
                b'T' => b'A',
                b'Q' => b'K',
                b'K' => b'Q',
                b'A' => b'T',
                _ => input[input_index],
            };

            input_index += 1;
        }

        // Skip " "
        input_index += 1;

        let bid_amount = read_value(input, &mut input_index);

        let mut sorted_hand = hand;
        sorted_hand.sort();

        let mut card_occurrences_vec = sorted_hand
            .group_by(|card_a, card_b| card_a == card_b)
            .map(|same_cards| same_cards.len())
            .collect::<Vec<_>>();

        // Sort card occurrences (descending)
        card_occurrences_vec.sort_by(|card_occurrences_a, card_occurrences_b| {
            card_occurrences_b.cmp(card_occurrences_a)
        });

        let r#type: u32 = match card_occurrences_vec.as_slice() {
            [5] => 6,        // Five of a kind
            [4, ..] => 5,    // Four of a kind
            [3, 2] => 4,     // Full house
            [3, ..] => 3,    // Three of a kind
            [2, 2, ..] => 2, // Two pair
            [2, ..] => 1,    // One pair
            _ => 0,          // High card
        };

        hands.push((hand, bid_amount, r#type));

        if input_index == input.len() {
            break;
        }

        // Skip "\n"
        input_index += 1;
    }

    // Sort hands by type and then by card (ascending)
    let mut packed_hands = hands
        .iter()
        .map(|hand| {
            let mut packed_hand;
            packed_hand = hand.1 as u64; // Bid amount
            packed_hand |= (hand.0[4] as u64) << 16; // Fifth card
            packed_hand |= (hand.0[3] as u64) << 24; // Fourth card
            packed_hand |= (hand.0[2] as u64) << 32; // Third card
            packed_hand |= (hand.0[1] as u64) << 40; // Second card
            packed_hand |= (hand.0[0] as u64) << 48; // First card
            packed_hand |= (hand.2 as u64) << 56; // Type

            packed_hand
        })
        .collect::<Vec<_>>();
    packed_hands.sort();

    // Calculate the total winnings
    packed_hands
        .iter()
        .enumerate()
        .map(|(hand_index, &hand)| {
            let unpacked_bid_amount = (hand & 0xFFFF) as u32;
            unpacked_bid_amount * (hand_index as u32 + 1)
        })
        .sum()
}

#[inline(always)]
fn part2() -> u32 {
    const REMAPPED_JACK: u8 = b'0' - 1;

    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut hands = Vec::new();

    loop {
        let mut hand = [0u8; 5];

        for card in &mut hand {
            // Remap ASCII chars in a way that preserves the strengths of the camel cards while making comparing
            // them easier
            *card = match input[input_index] {
                b'J' => REMAPPED_JACK,
                b'T' => b'A',
                b'Q' => b'K',
                b'K' => b'Q',
                b'A' => b'T',
                _ => input[input_index],
            };

            input_index += 1;
        }

        // Skip " "
        input_index += 1;

        let bid_amount = read_value(input, &mut input_index);

        let mut sorted_hand = hand;
        sorted_hand.sort();

        let mut num_jacks = 0;

        let mut card_occurrences_vec = sorted_hand
            .iter()
            .filter(|&&card| {
                if card != REMAPPED_JACK {
                    return true;
                }

                num_jacks += 1;

                false
            })
            .collect::<Vec<_>>()
            .group_by(|card_a, card_b| card_a == card_b)
            .map(|same_cards| same_cards.len())
            .collect::<Vec<_>>();

        // Sort card occurrences (descending)
        card_occurrences_vec.sort_by(|card_occurrences_a, card_occurrences_b| {
            card_occurrences_b.cmp(card_occurrences_a)
        });

        if num_jacks > 0 {
            if let Some(card_occurrences) = card_occurrences_vec.first_mut() {
                *card_occurrences += num_jacks;
            } else {
                card_occurrences_vec.push(num_jacks);
            }
        }

        let r#type: u32 = match card_occurrences_vec.as_slice() {
            [5] => 6,        // Five of a kind
            [4, ..] => 5,    // Four of a kind
            [3, 2] => 4,     // Full house
            [3, ..] => 3,    // Three of a kind
            [2, 2, ..] => 2, // Two pair
            [2, ..] => 1,    // One pair
            _ => 0,          // High card
        };

        hands.push((hand, bid_amount, r#type));

        if input_index == input.len() {
            break;
        }

        // Skip "\n"
        input_index += 1;
    }

    // Sort hands by type and then by card (ascending)
    let mut packed_hands = hands
        .iter()
        .map(|hand| {
            let mut packed_hand;
            packed_hand = hand.1 as u64; // Bid amount
            packed_hand |= (hand.0[4] as u64) << 16; // Fifth card
            packed_hand |= (hand.0[3] as u64) << 24; // Fourth card
            packed_hand |= (hand.0[2] as u64) << 32; // Third card
            packed_hand |= (hand.0[1] as u64) << 40; // Second card
            packed_hand |= (hand.0[0] as u64) << 48; // First card
            packed_hand |= (hand.2 as u64) << 56; // Type

            packed_hand
        })
        .collect::<Vec<_>>();
    packed_hands.sort();

    // Calculate the total winnings
    packed_hands
        .iter()
        .enumerate()
        .map(|(hand_index, &hand)| {
            let unpacked_bid_amount = (hand & 0xFFFF) as u32;
            unpacked_bid_amount * (hand_index as u32 + 1)
        })
        .sum()
}

#[inline(always)]
fn read_value(input: &[u8], input_index: &mut usize) -> u16 {
    let mut value = 0;

    while *input_index < input.len() && input[*input_index] != b'\n' {
        value *= 10u16;
        // The lower 4 bits of the ASCII char are matching the value it represents
        value += (input[*input_index] & 0x0F) as u16;

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
        assert_eq!(part1(), 250347426);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 251224870);
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

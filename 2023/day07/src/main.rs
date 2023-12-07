// https://adventofcode.com/2023/day/7
#![feature(test)]
#![feature(slice_group_by)]
extern crate test;

use std::{cmp::Ordering, collections::HashMap};

fn main() {
    part1();
    part2();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
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
    hands.sort_by(|hand_a, hand_b| match hand_a.2.cmp(&hand_b.2) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            for (card_index, card_a) in hand_a.0.iter().enumerate() {
                let card_b = hand_b.0.get(card_index).unwrap();

                match card_a.cmp(card_b) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Equal,
                    _ => {}
                }
            }

            Ordering::Equal
        }
    });

    // Calculate the total winnings
    hands
        .iter()
        .enumerate()
        .map(|(card_index, hand)| hand.1 * (card_index as u32 + 1))
        .sum()
}

#[inline(always)]
fn part2() -> u32 {
    let input = include_str!("../input");

    let mut hand_to_type_and_bid_amount = Vec::new();

    for line in input.lines() {
        let mut split = line.split(' ');
        let hand_str = split.next().unwrap();
        let hand = hand_str
            .chars()
            .map(|card_ch| Card::from(card_ch))
            .collect::<Vec<_>>();
        let bid_amount = split.next().unwrap().parse::<u32>().unwrap();

        if hand.contains(&Card::Jack) {
            let jack_indices = hand
                .iter()
                .enumerate()
                .filter(|&(_, card)| *card == Card::Jack)
                .map(|(jack_index, _)| jack_index)
                .collect::<Vec<_>>();

            let mut permutated_hands = Vec::new();
            let mut permutated_hand = hand.clone();

            let new_sub_hands = generate_permutated_cards(jack_indices.len());

            for new_sub_hand in &new_sub_hands {
                for (i, new_card) in new_sub_hand.iter().enumerate() {
                    let index = jack_indices[i];
                    permutated_hand[index] = *new_card;
                }

                permutated_hands.push(permutated_hand.clone());
            }

            let mut best_type = Type::HighCard;

            for permutated_hand in permutated_hands {
                let r#type;

                if is_x_of_a_kind(&permutated_hand, 5) {
                    r#type = Type::FiveOfAKind;
                } else if is_x_of_a_kind(&permutated_hand, 4) {
                    r#type = Type::FourOfAKind;
                } else if is_full_house(&permutated_hand) {
                    r#type = Type::FullHouse;
                } else if is_x_of_a_kind(&permutated_hand, 3) {
                    r#type = Type::ThreeOfAKind;
                } else if is_two_pair(&permutated_hand) {
                    r#type = Type::TwoPair;
                } else if is_one_pair(&permutated_hand) {
                    r#type = Type::OnePair;
                } else {
                    r#type = Type::HighCard;
                }

                if r#type > best_type {
                    best_type = r#type;
                }
            }

            hand_to_type_and_bid_amount.push((hand, (best_type, bid_amount)));
        } else {
            if is_x_of_a_kind(&hand, 5) {
                hand_to_type_and_bid_amount.push((hand, (Type::FiveOfAKind, bid_amount)));
            } else if is_x_of_a_kind(&hand, 4) {
                hand_to_type_and_bid_amount.push((hand, (Type::FourOfAKind, bid_amount)));
            } else if is_full_house(&hand) {
                hand_to_type_and_bid_amount.push((hand, (Type::FullHouse, bid_amount)));
            } else if is_x_of_a_kind(&hand, 3) {
                hand_to_type_and_bid_amount.push((hand, (Type::ThreeOfAKind, bid_amount)));
            } else if is_two_pair(&hand) {
                hand_to_type_and_bid_amount.push((hand, (Type::TwoPair, bid_amount)));
            } else if is_one_pair(&hand) {
                hand_to_type_and_bid_amount.push((hand, (Type::OnePair, bid_amount)));
            } else {
                hand_to_type_and_bid_amount.push((hand, (Type::HighCard, bid_amount)));
            }
        }
    }

    // Sort by type asc
    hand_to_type_and_bid_amount.sort_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap());

    // Check for hands with the same type
    let types = vec![
        Type::HighCard,
        Type::OnePair,
        Type::TwoPair,
        Type::ThreeOfAKind,
        Type::FullHouse,
        Type::FourOfAKind,
        Type::FiveOfAKind,
    ];
    let mut ranked_hand_to_type_and_bid_amount = Vec::new();

    for r#type in types {
        let mut hand_to_specific_type_and_bid_amount = hand_to_type_and_bid_amount
            .iter()
            .filter(|entry| entry.1 .0 == r#type)
            .map(|entry| entry.clone())
            .collect::<Vec<_>>();

        // Sort by card asc
        hand_to_specific_type_and_bid_amount.sort_by(|entry_a, entry_b| {
            for (card_index, card_a) in entry_a.0.iter().enumerate() {
                let card_b = entry_b.0.get(card_index).unwrap();

                if *card_a == Card::Jack && *card_b != Card::Jack {
                    return std::cmp::Ordering::Less;
                } else if *card_a != Card::Jack && *card_b == Card::Jack {
                    return std::cmp::Ordering::Greater;
                }

                if card_a < card_b {
                    return std::cmp::Ordering::Less;
                } else if card_a > card_b {
                    return std::cmp::Ordering::Greater;
                }
            }

            std::cmp::Ordering::Equal
        });

        ranked_hand_to_type_and_bid_amount.extend(hand_to_specific_type_and_bid_amount);
    }

    let mut total_winnings = 0;

    for (hand_index, hand_to_type_and_bid_amount) in
        ranked_hand_to_type_and_bid_amount.iter().enumerate()
    {
        let bid_amount = hand_to_type_and_bid_amount.1 .1;
        total_winnings += bid_amount * (hand_index as u32 + 1);
    }

    total_winnings
}

#[inline(always)]
fn read_value(input: &[u8], input_index: &mut usize) -> u32 {
    let mut value = 0;

    while *input_index < input.len() && input[*input_index] != b'\n' {
        value *= 10u32;
        // The lower 4 bits of the ASCII char are matching the value it represents
        value += (input[*input_index] & 0x0F) as u32;

        *input_index += 1;
    }

    value
}

fn generate_permutated_cards(length: usize) -> Vec<Vec<Card>> {
    let mut permutations = Vec::new();
    let mut current_permutation = Vec::new();

    generate_permutations(&mut permutations, &mut current_permutation, length);

    permutations
        .iter()
        .filter(|&permutation| permutation.len() == length)
        .map(|permutation| permutation.to_owned())
        .collect::<Vec<_>>()
}

fn generate_permutations(
    permutations: &mut Vec<Vec<Card>>,
    current_permutation: &mut Vec<Card>,
    length: usize,
) {
    if current_permutation.len() == length {
        permutations.push(current_permutation.to_vec());
        return;
    }

    let cards = vec![
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Jack,
        Card::Queen,
        Card::King,
        Card::Ace,
    ];

    for card in cards {
        let mut new_permutation = current_permutation.clone();
        new_permutation.push(card);

        generate_permutations(permutations, &mut new_permutation, length);
    }
}

fn get_card_occurrences(hand: &[Card]) -> HashMap<&Card, u8> {
    let mut occurrences_map = HashMap::new();

    for card in hand {
        if !occurrences_map.contains_key(card) {
            occurrences_map.insert(card, 1);
        } else {
            let occurrences = occurrences_map.get_mut(card).unwrap();
            *occurrences += 1;
        }
    }

    occurrences_map
}

fn is_x_of_a_kind(hand: &[Card], x: u8) -> bool {
    let occurrences = get_card_occurrences(hand);

    occurrences.values().any(|&occurrences| occurrences == x)
        && occurrences.len() == (-(x as i8) + 6) as usize
}

fn is_full_house(hand: &[Card]) -> bool {
    let occurrences_map = get_card_occurrences(hand);

    occurrences_map
        .values()
        .any(|&occurrences| occurrences == 3)
        && occurrences_map.len() == 2
}

fn is_two_pair(hand: &[Card]) -> bool {
    let occurrences_map = get_card_occurrences(hand);

    occurrences_map
        .values()
        .filter(|&&occurrences| occurrences == 2)
        .count()
        == 2
}

fn is_one_pair(hand: &[Card]) -> bool {
    let occurrences_map = get_card_occurrences(hand);

    occurrences_map
        .values()
        .filter(|&&occurrences| occurrences == 2)
        .count()
        == 1
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

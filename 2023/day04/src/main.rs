// https://adventofcode.com/2023/day/4
#![feature(test)]
extern crate test;

use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_str!("../input");

    let mut points = 0;

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
        let our_numbers = our_numbers_str
            .split(' ')
            .map(|str| str.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut num_winning_numbers = 0;

        for our_number in our_numbers {
            if their_card.contains(&our_number) {
                num_winning_numbers += 1;
            }
        }

        if num_winning_numbers > 0 {
            points += 2_u32.pow(num_winning_numbers - 1);
        }
    }

    points
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

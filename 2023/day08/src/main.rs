// https://adventofcode.com/2023/day/8
#![feature(test)]
extern crate test;

use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut instructions = Vec::new();

    while input[input_index] != b'\n' {
        instructions.push(input[input_index]);
        input_index += 1;
    }

    // Skip "\n\n"
    input_index += 2;

    // get_index(b"ZZZ") == 25 * 26^2 + 25 * 26 + 25 = 17575
    let mut node_mappings = [(0u16, 0u16); 25 * 26 * 26 + 25 * 26 + 25 + 1];

    loop {
        let from_node_index = get_combined_index(&input[input_index..input_index + 3]);

        // Skip "XXX = ("
        input_index += 7;

        let to_left_node_index = get_combined_index(&input[input_index..input_index + 3]);

        // Skip "XXX, "
        input_index += 5;

        let to_right_node_index = get_combined_index(&input[input_index..input_index + 3]);

        node_mappings[from_node_index] = (to_left_node_index as u16, to_right_node_index as u16);

        // Skip "XXX)" and a potential "\n"
        input_index += 5;

        if input_index >= input.len() {
            break;
        }
    }

    let mut node_index = get_combined_index(b"AAA");
    let end_node_index = get_combined_index(b"ZZZ");

    let mut steps = 0;

    'outer: loop {
        for &instruction in &instructions {
            let next_node_indices = node_mappings[node_index];

            if instruction == b'L' {
                node_index = next_node_indices.0 as usize;
            } else {
                node_index = next_node_indices.1 as usize;
            }

            steps += 1;

            if node_index == end_node_index {
                break 'outer;
            }
        }
    }

    steps
}

#[inline(always)]
fn part2() -> u64 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut instructions = Vec::new();

    while input[input_index] != b'\n' {
        instructions.push(input[input_index]);
        input_index += 1;
    }

    // Skip "\n\n"
    input_index += 2;

    // get_index(b"ZZZ") == 25 * 26^2 + 25 * 26 + 25 = 17575
    let mut node_mappings = [(0u16, 0u16); 25 * 26 * 26 + 25 * 26 + 25 + 1];
    let mut node_indices = Vec::new();

    loop {
        let from_node_index = get_combined_index(&input[input_index..input_index + 3]);

        if input[input_index + 2] == b'A' {
            node_indices.push(from_node_index);
        }

        // Skip "XXX = ("
        input_index += 7;

        let to_left_node_index = get_combined_index(&input[input_index..input_index + 3]);

        // Skip "XXX, "
        input_index += 5;

        let to_right_node_index = get_combined_index(&input[input_index..input_index + 3]);

        node_mappings[from_node_index] = (to_left_node_index as u16, to_right_node_index as u16);

        // Skip "XXX)" and a potential "\n"
        input_index += 5;

        if input_index >= input.len() {
            break;
        }
    }

    let mut end_nodes_visited = HashMap::new();
    let mut end_node_cycles = HashMap::new();
    let mut steps = 0u64;

    while end_node_cycles.len() < node_indices.len() {
        for &instruction in &instructions {
            for node_index in node_indices.iter_mut() {
                let next_node_indices = node_mappings[*node_index];

                if instruction == b'L' {
                    *node_index = next_node_indices.0 as usize;
                } else {
                    *node_index = next_node_indices.1 as usize;
                }

                if get_last_letter(*node_index) != 'Z' {
                    continue;
                }

                match end_nodes_visited.entry(*node_index) {
                    Occupied(occupied_entry) => {
                        if let Vacant(vacant_entry) = end_node_cycles.entry(*node_index) {
                            vacant_entry.insert(steps - occupied_entry.get());
                        }
                    }
                    Vacant(vacant_entry) => {
                        vacant_entry.insert(steps);
                    }
                }
            }

            steps += 1;
        }
    }

    // Calculate the least common multiple (LCM)
    end_node_cycles
        .values()
        .copied()
        .reduce(num::integer::lcm)
        .unwrap()
}

#[inline(always)]
fn get_combined_index(slice: &[u8]) -> usize {
    // We are shifting the letters so that the resulting combined index does not get too big
    (slice[0] - b'A') as usize * 26 * 26
        + (slice[1] - b'A') as usize * 26
        + (slice[2] - b'A') as usize
}

#[inline(always)]
fn get_last_letter(combined_index: usize) -> char {
    ((combined_index % 26) as u8 + b'A') as char
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 21883);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 12833235391111);
    }

    #[test]
    fn test_get_combined_index() {
        assert_eq!(get_combined_index(b"AAA"), 0);
        assert_eq!(get_combined_index(b"WTS"), 15384);
        assert_eq!(get_combined_index(b"ZZZ"), 17575);
    }

    #[test]
    fn test_get_last_letter() {
        assert_eq!(get_last_letter(get_combined_index(b"AAA")), 'A');
        assert_eq!(get_last_letter(get_combined_index(b"WTS")), 'S');
        assert_eq!(get_last_letter(get_combined_index(b"ZZZ")), 'Z');
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

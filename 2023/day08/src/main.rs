// https://adventofcode.com/2023/day/8
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

    let mut instructions = Vec::new();
    let mut node_mappings = HashMap::new();

    for (line_index, line) in input.lines().enumerate() {
        if line_index == 0 {
            instructions = line.chars().collect::<Vec<_>>();
            continue;
        }

        if line_index == 1 {
            continue;
        }

        let mut split = line.split(" = ");
        let from_node = split.next().unwrap();
        let to_nodes = split.next().unwrap().replace(['(', ')'], "");

        let mut split2 = to_nodes.split(", ");
        let to_left_node = split2.next().unwrap();
        let to_right_node = split2.next().unwrap();

        node_mappings.insert(
            from_node.to_string(),
            (to_left_node.to_string(), to_right_node.to_string()),
        );
    }

    let mut node = "AAA".to_string();
    let mut steps = 0;

    'outer: loop {
        for &instruction in &instructions {
            let next_nodes = node_mappings.get(&node).unwrap();

            if instruction == 'L' {
                node = next_nodes.0.clone();
            } else {
                node = next_nodes.1.clone();
            }

            steps += 1;

            if node == "ZZZ" {
                break 'outer;
            }
        }
    }

    steps
}

#[inline(always)]
fn part2() -> u64 {
    let input = include_str!("../input");

    let mut instructions = Vec::new();
    let mut nodes = Vec::new();
    let mut node_mappings = HashMap::new();

    for (line_index, line) in input.lines().enumerate() {
        if line_index == 0 {
            instructions = line.chars().collect::<Vec<_>>();
            continue;
        }

        if line_index == 1 {
            continue;
        }

        let mut split = line.split(" = ");
        let from_node = split.next().unwrap();
        let to_nodes = split.next().unwrap().replace(['(', ')'], "");

        let mut split2 = to_nodes.split(", ");
        let to_left_node = split2.next().unwrap();
        let to_right_node = split2.next().unwrap();

        if from_node.ends_with('A') {
            nodes.push(from_node.to_string());
        }

        node_mappings.insert(
            from_node.to_string(),
            (to_left_node.to_string(), to_right_node.to_string()),
        );
    }

    let mut end_nodes_visited = HashMap::new();
    let mut end_node_cycles = HashMap::new();
    let mut steps = 0u64;

    while end_node_cycles.len() < nodes.len() {
        for &instruction in &instructions {
            for node in nodes.iter_mut() {
                let next_nodes: &(String, String) = node_mappings.get(&node.to_owned()).unwrap();

                if instruction == 'L' {
                    *node = next_nodes.0.clone();
                } else {
                    *node = next_nodes.1.clone();
                }

                if node.ends_with("Z") {
                    if !end_nodes_visited.contains_key(node) {
                        println!("Visited an end node! (end node: {node}, steps: {steps})");
                        end_nodes_visited.insert(node.clone(), steps);
                    } else if !end_node_cycles.contains_key(node) {
                        println!(
                            "Found an end node cycle! (end node: {node}, {steps} - {} = {})",
                            end_nodes_visited[node],
                            steps - end_nodes_visited[node]
                        );
                        end_node_cycles.insert(node.clone(), steps - end_nodes_visited[node]);
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

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| black_box(part1()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| black_box(part2()));
    }
}

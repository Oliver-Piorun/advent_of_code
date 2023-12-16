// https://adventofcode.com/2023/day/3
#![feature(test)]
extern crate test;

use std::cmp::{max, min};

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u32 {
    let input = include_str!("../input");
    let engine_schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = engine_schematic.len();
    let width = engine_schematic.first().unwrap().len();

    let mut part_number_sum = 0;
    let mut part_number = 0;
    let mut found_adjacent_symbol = false;

    for i in 0..height {
        for j in 0..width {
            let char = engine_schematic[i][j];

            if char.is_ascii_digit() {
                // Dealing with a digit
                part_number *= 10u32;
                part_number += (char as u8 - b'0') as u32;

                if !found_adjacent_symbol {
                    // Search for a symbol which is adjacent to the current part number digit
                    let k_from = max(0, i as isize - 1) as usize;
                    let k_to = min(i + 1, height - 1);
                    let l_from = max(0, j as isize - 1) as usize;
                    let l_to = min(j + 1, width - 1);

                    #[allow(clippy::needless_range_loop)]
                    'outer: for k in k_from..=k_to {
                        for l in l_from..=l_to {
                            let adjacent_char = engine_schematic[k][l];

                            if !adjacent_char.is_ascii_digit() && adjacent_char != '.' {
                                found_adjacent_symbol = true;
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if !char.is_ascii_digit() || j == width - 1 {
                // Not dealing with a digit or we are at the end of a row
                if found_adjacent_symbol {
                    part_number_sum += part_number;
                    found_adjacent_symbol = false;
                }

                part_number = 0;
            }
        }
    }

    part_number_sum
}

#[inline(always)]
fn part2() -> u32 {
    let input = include_str!("../input");
    let engine_schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = engine_schematic.len();
    let width = engine_schematic.first().unwrap().len();

    let mut gear_ratio_sum = 0;

    let mut first_part_number = 0;
    let mut second_part_number = 0;
    let mut visited_digits = Vec::new();
    let mut found_adjacent_asterisk = false;
    let mut adjacent_asterisk_k = 0;
    let mut adjacent_asterisk_l = 0;
    let mut visited_adjacent_asterisks = Vec::new();

    for i in 0..height {
        for j in 0..width {
            let char = engine_schematic[i][j];

            if char.is_ascii_digit() {
                // Dealing with a digit
                first_part_number *= 10u32;
                first_part_number += (char as u8 - b'0') as u32;
                visited_digits.push((i, j));

                if !found_adjacent_asterisk {
                    // Search for "*" which is adjacent to the current part number digit
                    let k_from = max(0, i as isize - 1) as usize;
                    let k_to = min(i + 1, height - 1);
                    let l_from = max(0, j as isize - 1) as usize;
                    let l_to = min(j + 1, width - 1);

                    #[allow(clippy::needless_range_loop)]
                    'outer: for k in k_from..=k_to {
                        for l in l_from..=l_to {
                            if visited_adjacent_asterisks.contains(&(k, l)) {
                                continue;
                            }

                            let adjacent_char = engine_schematic[k][l];

                            if adjacent_char == '*' {
                                found_adjacent_asterisk = true;
                                adjacent_asterisk_k = k;
                                adjacent_asterisk_l = l;
                                visited_adjacent_asterisks.push((k, l));
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if !char.is_ascii_digit() || j == width - 1 {
                // Not dealing with a digit or we are at the end of a row
                if found_adjacent_asterisk {
                    let mut found_exactly_one_second_part_number = false;

                    // Search for an unvisited digit which is adjacent to the found "*"
                    let k_from = max(0, adjacent_asterisk_k as isize - 1) as usize;
                    let k_to = min(adjacent_asterisk_k + 1, height - 1);
                    let l_from = max(0, adjacent_asterisk_l as isize - 1) as usize;
                    let l_to = min(adjacent_asterisk_l + 1, width - 1);

                    #[allow(clippy::needless_range_loop)]
                    'outer: for k in k_from..=k_to {
                        for l in l_from..=l_to {
                            if visited_digits.contains(&(k, l)) {
                                continue;
                            }

                            let adjacent_char = engine_schematic[k][l];

                            if adjacent_char.is_ascii_digit() {
                                if found_exactly_one_second_part_number {
                                    found_exactly_one_second_part_number = false;
                                    break 'outer;
                                }

                                found_exactly_one_second_part_number = true;

                                let mut char_m_from = l as isize;

                                while char_m_from != -1
                                    && engine_schematic[k][char_m_from as usize].is_ascii_digit()
                                {
                                    char_m_from -= 1;
                                }

                                char_m_from += 1;

                                for m in char_m_from as usize..width {
                                    let char = engine_schematic[k][m];

                                    if char.is_ascii_digit() {
                                        visited_digits.push((k, m));
                                        second_part_number *= 10u32;
                                        second_part_number += (char as u8 - b'0') as u32;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if found_exactly_one_second_part_number {
                        gear_ratio_sum += first_part_number * second_part_number;
                    }

                    second_part_number = 0;
                    found_adjacent_asterisk = false;
                    adjacent_asterisk_k = 0;
                    adjacent_asterisk_l = 0;
                }

                first_part_number = 0;
                visited_digits.clear();
            }
        }
    }

    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 535235);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 79844424);
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

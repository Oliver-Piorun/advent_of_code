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
    let input = include_bytes!("../input");

    let width = input
        .iter()
        .position(|&character| character == b'\n')
        .unwrap();
    let height = input.len() / (width + 1) + 1;

    let mut part_number_sum = 0;
    let mut part_number = 0;
    let mut found_adjacent_symbol = false;

    for y in 0..height {
        for x in 0..width {
            let character = get_character(y, x, input, width);

            if character.is_ascii_digit() {
                // Dealing with a digit
                part_number = part_number * 10 + (character & 0xF) as u32;

                if !found_adjacent_symbol {
                    // Search for a symbol which is adjacent to the current part number digit
                    let i_from = max(0, y as isize - 1) as usize;
                    let i_to = min(y + 1, height - 1);
                    let j_from = max(0, x as isize - 1) as usize;
                    let j_to = min(x + 1, width - 1);

                    #[allow(clippy::needless_range_loop)]
                    'outer: for k in i_from..=i_to {
                        for l in j_from..=j_to {
                            let adjacent_character = get_character(k, l, input, width);

                            if adjacent_character != b'.' && !adjacent_character.is_ascii_digit() {
                                found_adjacent_symbol = true;
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if !character.is_ascii_digit() || x == width - 1 {
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
    let input = include_bytes!("../input");

    let width = input
        .iter()
        .position(|&character| character == b'\n')
        .unwrap();
    let height = input.len() / (width + 1) + 1;

    let mut gear_ratio_sum = 0;

    for y in 0..height {
        for x in 0..width {
            let character = get_character(y, x, input, width);

            if character == b'*' {
                let mut part_numbers = Vec::new();

                if y != 0 {
                    // Take a look at the row above
                    get_part_numbers(y - 1, x, &mut part_numbers, input, width);
                }

                // Take a look at the current row
                get_part_numbers(y, x, &mut part_numbers, input, width);

                if y != height - 1 {
                    // Take a look at the row below
                    get_part_numbers(y + 1, x, &mut part_numbers, input, width);
                }

                if part_numbers.len() == 2 {
                    gear_ratio_sum += part_numbers[0] as u32 * part_numbers[1] as u32;
                }
            }
        }
    }

    gear_ratio_sum
}

#[inline(always)]
fn get_character(y: usize, x: usize, input: &[u8], width: usize) -> u8 {
    input[y * (width + 1) + x]
}

#[inline(always)]
fn get_part_numbers(y: usize, x: usize, part_numbers: &mut Vec<u16>, input: &[u8], width: usize) {
    let mut current_x = x as isize;
    let mut found_digits = false;

    // Try to follow left characters which are digits
    loop {
        current_x -= 1;

        if current_x < 0 || !get_character(y, current_x as usize, input, width).is_ascii_digit() {
            break;
        }

        found_digits = true;
    }

    current_x += 1;

    if found_digits {
        // Left characters are digits
        part_numbers.push(read_value(y, &mut current_x, input, width));
    }

    // We are at the middle character
    if (current_x as usize) == x {
        if get_character(y, current_x as usize, input, width).is_ascii_digit() {
            // Middle character is a digit
            part_numbers.push(read_value(y, &mut current_x, input, width))
        } else if get_character(y, (current_x + 1) as usize, input, width).is_ascii_digit() {
            // Middle character is not a digit but the right character is
            current_x += 1;
            part_numbers.push(read_value(y, &mut current_x, input, width));
        }
    }
}

#[inline(always)]
fn read_value(y: usize, x: &mut isize, input: &[u8], width: usize) -> u16 {
    let mut value = 0;

    loop {
        if *x as usize == width {
            break;
        }

        let character = get_character(y, *x as usize, input, width);

        if !character.is_ascii_digit() {
            break;
        }

        value = value * 10 + (character & 0xF) as u16;

        *x += 1;
    }

    value
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

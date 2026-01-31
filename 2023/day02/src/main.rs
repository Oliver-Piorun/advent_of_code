// https://adventofcode.com/2023/day/2
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

    let mut possible_game_id_sum = 0u32;

    'line_loop: loop {
        // Skip "Game "
        input_index += 5;

        let game_id = read_value(input, &mut input_index);

        // Skip ": "
        input_index += 2;

        let mut is_possible_game = true;

        loop {
            let value = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;

            if input[input_index] == b'r' && value > 12
                || input[input_index] == b'g' && value > 13
                || value > 14
            {
                is_possible_game = false;
            }

            // Skip "r", "g" or "b"
            input_index += 1;

            loop {
                if input_index != input.len() && input[input_index] != b'\n' {
                    if is_possible_game {
                        if input[input_index] != b',' && input[input_index] != b';' {
                            input_index += 1;
                        } else {
                            // Skip ", " or "; "
                            input_index += 2;
                            break;
                        }
                    } else {
                        input_index += 1;
                    }
                } else {
                    if is_possible_game {
                        possible_game_id_sum += game_id as u32;
                    }

                    if input_index == input.len() {
                        break 'line_loop;
                    } else {
                        // Skip "\n"
                        input_index += 1;
                        continue 'line_loop;
                    }
                }
            }
        }
    }

    possible_game_id_sum
}

#[inline(always)]
fn part2() -> u32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut power_sum = 0u32;

    'line_loop: loop {
        // Skip "Game "
        input_index += 5;

        while input[input_index] != b':' {
            input_index += 1;
        }

        // Skip ": "
        input_index += 2;

        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        loop {
            let value = read_value(input, &mut input_index);

            // Skip " "
            input_index += 1;

            match input[input_index] {
                b'r' => {
                    if value > red_max {
                        red_max = value;
                    }
                }
                b'g' => {
                    if value > green_max {
                        green_max = value;
                    }
                }
                _ => {
                    if value > blue_max {
                        blue_max = value;
                    }
                }
            }

            // Skip "r", "g" or "b"
            input_index += 1;

            loop {
                if input_index != input.len() && input[input_index] != b'\n' {
                    if input[input_index] != b',' && input[input_index] != b';' {
                        input_index += 1;
                    } else {
                        // Skip ", " or "; "
                        input_index += 2;
                        break;
                    }
                } else {
                    power_sum += red_max as u32 * green_max as u32 * blue_max as u32;

                    if input_index == input.len() {
                        break 'line_loop;
                    } else {
                        // Skip "\n"
                        input_index += 1;
                        continue 'line_loop;
                    }
                }
            }
        }
    }

    power_sum
}

#[inline]
fn read_value(input: &[u8], index: &mut usize) -> u8 {
    let mut value = 0;

    while input[*index] >= b'0' && input[*index] <= b'9' {
        value *= 10;
        value += input[*index] - b'0';

        *index += 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2164);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 69929);
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

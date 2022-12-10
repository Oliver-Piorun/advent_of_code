// https://adventofcode.com/2022/day/10
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut cycle = 0;
    let mut register_x = 1;
    let mut signal_strength_sums = 0;

    while input_index < input.len() {
        if input[input_index] == b'n' {
            // noop
            // Skip "noop" or "noop\n"
            skip_past_line_end(input, &mut input_index);

            cycle += 1;

            if cycle >= 20 && ((cycle - 20) % 40 == 0) {
                signal_strength_sums += cycle * register_x;
            }
        } else {
            // addx
            for i in 0..2 {
                cycle += 1;

                if cycle >= 20 && ((cycle - 20) % 40 == 0) {
                    signal_strength_sums += cycle * register_x;
                }

                if i == 1 {
                    register_x += read_value(input, &mut input_index);
                }
            }
        }
    }

    signal_strength_sums
}

#[inline(always)]
fn part2() -> [[char; 40]; 6] {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    let mut cycle = 0;
    let mut register_x = 1;
    let mut crt = [['.'; 40]; 6];

    while input_index < input.len() {
        if input[input_index] == b'n' {
            // noop
            // Skip "noop" or "noop\n"
            skip_past_line_end(input, &mut input_index);

            cycle += 1;
            update_crt(&mut crt, cycle, register_x);
        } else {
            // addx
            for i in 0..2 {
                cycle += 1;
                update_crt(&mut crt, cycle, register_x);

                if i == 1 {
                    register_x += read_value(input, &mut input_index);
                }
            }
        }
    }

    crt
}

#[inline(always)]
fn update_crt(crt: &mut [[char; 40]; 6], cycle: i32, register_x: i32) {
    let cycle_index = cycle - 1;
    let cycle_line_index = cycle_index % 40;

    for register_x_line_index in register_x - 1..=register_x + 1 {
        if register_x_line_index == cycle_line_index {
            crt[(cycle_index / 40) as usize][(cycle_index % 40) as usize] = '#';
        }
    }
}

#[inline(always)]
fn read_value(input: &[u8], index: &mut usize) -> i32 {
    // Skip "addx "
    *index += 5;

    let mut factor = 1;
    let mut value = 0;

    while *index < input.len() && input[*index] != b'\n' {
        if input[*index] == b'-' {
            factor = -1;
        } else {
            value = value * 10 + (input[*index] - b'0') as i32;
        }

        *index += 1;
    }

    // Skip a potential "\n"
    *index += 1;

    factor * value
}

#[inline(always)]
fn skip_past_line_end(input: &[u8], index: &mut usize) {
    while *index < input.len() {
        *index += 1;

        if input[*index - 1] == b'\n' {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 16060);
    }

    #[test]
    fn test_part2() {
        let crt = part2();

        // Run "cargo test -- --nocapture" to see the output
        for i in 0..crt.len() {
            for j in 0..crt[i].len() {
                print!("{}", crt[i][j]);
            }

            println!("\n");
        }

        let expected_output = include_str!("../expected_output");
        let mut expected_crt = [['\0'; 40]; 6];

        for (i, line) in expected_output.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                expected_crt[i][j] = c;
            }
        }

        assert_eq!(crt, expected_crt);
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

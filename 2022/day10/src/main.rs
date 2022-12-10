// https://adventofcode.com/2022/day/10
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_str!("../input");

    let mut cycle = 0;
    let mut register_x = 1;
    let mut signal_strength_sums = 0;

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;

            if cycle >= 20 && ((cycle - 20) % 40 == 0) {
                signal_strength_sums += cycle * register_x;
            }
        } else {
            for i in 0..2 {
                cycle += 1;

                if cycle >= 20 && ((cycle - 20) % 40 == 0) {
                    signal_strength_sums += cycle * register_x;
                }

                if i == 1 {
                    let mut split = line.split(' ');
                    split.next();
                    let value = split.next().unwrap().parse::<i32>().unwrap();
                    register_x += value;
                }
            }
        }
    }

    signal_strength_sums
}

#[inline(always)]
fn part2() -> [[char; 40]; 6] {
    let input = include_str!("../input");

    let mut cycle = 0;
    let mut register_x = 1;
    let mut crt = [['.'; 40]; 6];

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            update_crt(&mut crt, cycle, register_x);
        } else {
            for i in 0..2 {
                cycle += 1;
                update_crt(&mut crt, cycle, register_x);

                if i == 1 {
                    let mut split = line.split(' ');
                    split.next();
                    let value = split.next().unwrap().parse::<i32>().unwrap();
                    register_x += value;
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

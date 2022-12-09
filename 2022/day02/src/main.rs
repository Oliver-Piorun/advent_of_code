// https://adventofcode.com/2022/day/2
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_bytes!("../input");
    let mut total_score = 0;

    // Align 'A' and 'X'
    // 'X' (88) - 'A' (65) = 23
    let alignment = b'X' - b'A';

    for i in (0..input.len()).step_by(4) {
        let op_shape = input[i];
        let my_shape = input[i + 2];

        let score = match (op_shape, my_shape) {
            // Rock, Scissors
            (b'A', b'Z') => 0,
            // Scissors, Rock
            (b'C', b'X') => 6,
            // Lose
            _ if op_shape > my_shape - alignment => 0,
            // Win
            _ if op_shape < my_shape - alignment => 6,
            // Draw
            _ => 3,
        };

        // Align my_shape and 'X'
        // 'X' (88) - 'X' (88) = 0
        // 'Y' (89) - 'X' (88) = 1
        // 'Z' (90) - 'X' (88) = 2
        total_score += (my_shape - b'X' + 1) as i32 + score;
    }

    total_score
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_bytes!("../input");
    let mut total_score = 0;

    for i in (0..input.len()).step_by(4) {
        let op_shape = input[i];
        let exp_outcome = input[i + 2];

        let score = match (op_shape, exp_outcome) {
            // Rock, Lose => Scissors
            (b'A', b'X') => 3,
            // Scissors, Win => Rock
            (b'C', b'Z') => 1 + 6,
            // Any, Lose
            (_, b'X') => (op_shape - b'A' + 1) as i32 - 1,
            // Any, Draw
            (_, b'Y') => (op_shape - b'A' + 1) as i32 + 3,
            // Any, Win
            (_, b'Z') => (op_shape - b'A' + 1) as i32 + 1 + 6,
            _ => panic!("We should never land here!"),
        };

        total_score += score;
    }

    total_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 12535);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 15457);
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

#![feature(test)]
extern crate test;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lose = 1,
    Draw = 2,
    Win = 3,
}

// The idiotmatic way would be to implement the "FromStr" trait here but we keep it a bit simpler/shorter
fn to_shape(str: &str) -> Shape {
    match str {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Unexpected shape!"),
    }
}

fn to_outcome(str: &str) -> Outcome {
    match str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unexpected outcome!"),
    }
}

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let input = include_str!("../input");
    let mut total_score = 0;

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let op_shape = to_shape(split.first().unwrap());
        let my_shape = to_shape(split.get(1).unwrap());

        let score = match (op_shape, my_shape) {
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 6,
            _ => {
                if op_shape > my_shape {
                    0
                } else if op_shape < my_shape {
                    6
                } else {
                    3
                }
            }
        };

        total_score += my_shape as i32 + score;
    }

    println!("part1: {total_score}");

    total_score
}

fn part2() -> i32 {
    let input = include_str!("../input");
    let mut total_score = 0;

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let op_shape = to_shape(split.first().unwrap());
        let exp_outcome = to_outcome(split.get(1).unwrap());

        let score = match (op_shape, exp_outcome) {
            (Shape::Rock, Outcome::Lose) => Shape::Scissors as i32,
            (Shape::Scissors, Outcome::Win) => Shape::Rock as i32 + 6,
            (_, Outcome::Lose) => op_shape as i32 - 1,
            (_, Outcome::Draw) => op_shape as i32 + 3,
            (_, Outcome::Win) => op_shape as i32 + 1 + 6,
        };

        total_score += score;
    }

    println!("part2: {total_score}");

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

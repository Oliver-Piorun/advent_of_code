// https://adventofcode.com/2022/day/6
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

fn part1() -> usize {
    get_num_chars(4)
}

fn part2() -> usize {
    get_num_chars(14)
}

#[inline(always)]
fn get_num_chars(marker_length: usize) -> usize {
    let input = include_bytes!("../input");
    let mut stash = Vec::new();

    for (i, c) in input.iter().enumerate() {
        if !stash.contains(c) {
            stash.push(input[i]);

            if stash.len() == marker_length {
                return i + 1;
            }
        } else {
            let pos = stash.iter().position(|stashed_c| stashed_c == c).unwrap();
            stash.drain(..=pos);
            stash.push(input[i]);
        }
    }

    panic!("We should never land here!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1909);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3380);
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

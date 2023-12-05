// https://adventofcode.com/2023/day/5
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> u64 {
    let input = include_str!("../input");

    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    let split = input.split("\n\n");

    for (i, block) in split.enumerate() {
        if i == 0 {
            let mut split = block.split(": ");
            split.next();

            let second = split.next().unwrap();
            seeds = second
                .split(" ")
                .map(|number_as_str| number_as_str.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            continue;
        }

        let split = block.split("\n").skip(1);

        let mut src_dst_len_vec = Vec::new();

        for line in split {
            let dst_src_len = line
                .split(" ")
                .map(|str| str.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            src_dst_len_vec.push((dst_src_len[1], dst_src_len[0], dst_src_len[2]));
        }

        maps.push(src_dst_len_vec);
    }

    let mut current_dst;
    let mut min_current_dst = u64::MAX;

    for seed in seeds {
        current_dst = seed;

        for map in &maps {
            for src_dst_len in map {
                let src = src_dst_len.0;
                let dst = src_dst_len.1;
                let len = src_dst_len.2;

                let min = src;
                let max = src + len - 1;

                if current_dst >= min && current_dst <= max {
                    let diff = current_dst - min;
                    current_dst = dst + diff;
                    break;
                }
            }
        }

        if current_dst < min_current_dst {
            min_current_dst = current_dst;
        }
    }

    min_current_dst
}

#[inline(always)]
fn part2() -> u64 {
    let input = include_str!("../input");

    let mut numbers = Vec::new();
    let mut maps = Vec::new();

    let split = input.split("\n\n");

    for (i, block) in split.enumerate() {
        if i == 0 {
            let mut split = block.split(": ");
            split.next();

            let second = split.next().unwrap();
            numbers = second
                .split(" ")
                .map(|number_as_str| number_as_str.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            continue;
        }

        let split = block.split("\n").skip(1);

        let mut src_dst_len_vec = Vec::new();

        for line in split {
            let dst_src_len = line
                .split(" ")
                .map(|str| str.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            src_dst_len_vec.push((dst_src_len[1], dst_src_len[0], dst_src_len[2]));
        }

        maps.push(src_dst_len_vec);
    }

    let mut current_dst;
    let mut min_current_dst = u64::MAX;

    for start_seed_len in numbers.chunks_exact(2) {
        let start_seed = start_seed_len[0];
        let len = start_seed_len[1];

        println!("Start seed: {start_seed}, len: {len}");

        for seed in start_seed..start_seed + len {
            if seed % 1_000_0000 == 0 {
                println!("Seed: {seed}");
            }

            current_dst = seed;

            for map in &maps {
                for src_dst_len in map {
                    let src = src_dst_len.0;
                    let dst = src_dst_len.1;
                    let len = src_dst_len.2;

                    let min = src;
                    let max = src + len - 1;

                    if current_dst >= min && current_dst <= max {
                        let diff = current_dst - min;
                        current_dst = dst + diff;
                        break;
                    }
                }
            }

            if current_dst < min_current_dst {
                min_current_dst = current_dst;
            }
        }
    }

    println!("{min_current_dst}");

    min_current_dst
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 403695602);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 219529182);
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

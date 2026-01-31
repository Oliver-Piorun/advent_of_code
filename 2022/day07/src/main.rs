// https://adventofcode.com/2022/day/7
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_bytes!("../input");
    let mut dir_sizes = Vec::<i32>::new();

    get_dir_size(input, &mut 0, &mut dir_sizes);
    let total_dir_size_lte_100k = dir_sizes
        .iter()
        .filter(|&&dir_size| dir_size <= 100_000)
        .sum();

    total_dir_size_lte_100k
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_bytes!("../input");
    let mut dir_sizes = Vec::<i32>::new();

    let total_dir_size = get_dir_size(input, &mut 0, &mut dir_sizes);
    let mut min_dir_size = i32::MAX;

    for dir_size in dir_sizes {
        if (70_000_000 - total_dir_size + dir_size) >= 30_000_000 && (dir_size < min_dir_size) {
            min_dir_size = dir_size;
        }
    }

    min_dir_size
}

#[inline(always)]
fn get_dir_size(input: &[u8], index: &mut usize, dir_sizes: &mut Vec<i32>) -> i32 {
    let mut dir_size = 0;

    while *index < input.len() {
        // Read 4 bytes
        let command_substr = &input[*index..*index + 4];

        if command_substr == b"$ cd" {
            // Skip 4 bytes and " "
            *index += 5;

            // Read 2 bytes
            let dir_substr = &input[*index..*index + 2];

            if dir_substr != b".." {
                skip_past_line_end(input, index);

                if *index == input.len() {
                    return dir_size;
                }

                let child_dir_size = get_dir_size(input, index, dir_sizes);
                dir_size += child_dir_size;

                dir_sizes.push(child_dir_size);
            } else {
                // Skip ".." and a potential "\n"
                *index += 3;
                return dir_size;
            }
        } else if command_substr == b"$ ls" {
            // Skip 4 bytes and a potential "\n"
            *index += 5;
        } else if command_substr == b"dir " {
            skip_past_line_end(input, index);
        } else {
            let mut file_size = 0i32;

            while input[*index] != b' ' {
                file_size = file_size * 10 + (input[*index] - b'0') as i32;
                *index += 1;
            }

            dir_size += file_size;
            skip_past_line_end(input, index);
        }
    }

    dir_size
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
    use test::{Bencher, black_box};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1886043);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3842121);
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

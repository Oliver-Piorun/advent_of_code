// https://adventofcode.com/2022/day/6
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let lines = include_str!("../input")
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();
    let mut total_dir_size_lte_100k = 0;

    get_dir_size_part1(&lines, &mut 0, &mut total_dir_size_lte_100k);

    total_dir_size_lte_100k
}

fn part2() -> i32 {
    let lines = include_str!("../input")
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();
    let mut dir_sizes = Vec::<i32>::new();

    let total_dir_size = get_dir_size_part2(&lines, &mut 0, &mut dir_sizes);
    let mut min_dir_size = Option::None::<i32>;

    for dir_size in dir_sizes {
        if (70_000_000 - total_dir_size + dir_size) >= 30_000_000
            && (min_dir_size.is_none() || dir_size < min_dir_size.unwrap())
        {
            min_dir_size = Some(dir_size);
        }
    }

    min_dir_size.unwrap()
}

fn get_dir_size_part1(
    lines: &[String],
    index: &mut usize,
    total_dir_size_lte_100k: &mut i32,
) -> i32 {
    let mut dir_size = 0;

    while *index < lines.len() {
        let line = lines.get(*index).unwrap();
        *index += 1;

        if line.contains("$ cd") {
            let dir = *line.split(' ').collect::<Vec<&str>>().get(2).unwrap();

            if dir != ".." {
                let child_dir_size = get_dir_size_part1(lines, index, total_dir_size_lte_100k);
                dir_size += child_dir_size;

                if child_dir_size <= 100_000 {
                    *total_dir_size_lte_100k += child_dir_size;
                }
            } else {
                return dir_size;
            }
        } else if line == "$ ls" || line.contains("dir ") {
            // Ignore
        } else {
            let file_size = line.split(' ').next().unwrap().parse::<i32>().unwrap();
            dir_size += file_size;
        }
    }

    dir_size
}

fn get_dir_size_part2(lines: &[String], index: &mut usize, dir_sizes: &mut Vec<i32>) -> i32 {
    let mut dir_size = 0;

    while *index < lines.len() {
        let line = lines.get(*index).unwrap();
        *index += 1;

        if line.contains("$ cd") {
            let dir = *line.split(' ').collect::<Vec<&str>>().get(2).unwrap();

            if dir != ".." {
                let child_dir_size = get_dir_size_part2(lines, index, dir_sizes);
                dir_size += child_dir_size;

                dir_sizes.push(child_dir_size);
            } else {
                return dir_size;
            }
        } else if line == "$ ls" || line.contains("dir ") {
            // Ignore
        } else {
            let file_size = line.split(' ').next().unwrap().parse::<i32>().unwrap();
            dir_size += file_size;
        }
    }

    dir_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

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

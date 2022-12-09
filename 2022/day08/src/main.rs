// https://adventofcode.com/2022/day/8
#![feature(test)]
extern crate test;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> i32 {
    let input = include_bytes!("../input");
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / width;

    let mut visible_tree_count = AtomicI32::new(0);

    (1..height - 1).into_par_iter().for_each(|i| {
        for j in 1..width - 1 {
            let tree = get_tree(input, i, j, width);
            let mut visible = true;

            // Top
            for k in (0..=i - 1).rev() {
                if get_tree(input, k, j, width) >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_tree_count.fetch_add(1, Ordering::SeqCst);
                continue;
            }

            visible = true;

            // Bottom
            for k in i + 1..height {
                if get_tree(input, k, j, width) >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_tree_count.fetch_add(1, Ordering::SeqCst);
                continue;
            }

            visible = true;

            // Left
            for k in (0..=j - 1).rev() {
                if get_tree(input, i, k, width) >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_tree_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                continue;
            }

            visible = true;

            // Right
            for k in j + 1..width {
                if get_tree(input, i, k, width) >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_tree_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    });

    let visible_tree_count = visible_tree_count.get_mut();
    *visible_tree_count += height as i32 * 2 + (width as i32 - 2) * 2;

    *visible_tree_count
}

#[inline(always)]
fn part2() -> i32 {
    let input = include_bytes!("../input");
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / width;

    let mut max_scenic_score = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let tree = get_tree(input, i, j, width);
            let mut top = 0;
            let mut bottom = 0;
            let mut left = 0;
            let mut right = 0;

            // Top
            for k in (0..=i - 1).rev() {
                top += 1;

                if get_tree(input, k, j, width) >= tree {
                    break;
                }
            }

            // Bottom
            for k in (i + 1)..height {
                bottom += 1;

                if get_tree(input, k, j, width) >= tree {
                    break;
                }
            }

            // Left
            for k in (0..=j - 1).rev() {
                left += 1;

                if get_tree(input, i, k, width) >= tree {
                    break;
                }
            }

            // Right
            for k in (j + 1)..width {
                right += 1;

                if get_tree(input, i, k, width) >= tree {
                    break;
                }
            }

            let scenic_score = top * bottom * left * right;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

#[inline(always)]
fn get_tree(input: &[u8], i: usize, j: usize, width: usize) -> u8 {
    input[i * (width + 1) + j] - b'0'
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1698);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 672280);
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

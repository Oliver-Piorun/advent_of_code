// https://adventofcode.com/2022/day/8
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let input = include_str!("../input");
    let mut trees = Vec::<Vec<u8>>::new();

    let mut visible_tree_count = 0;

    for (i, line) in input.lines().enumerate() {
        trees.push(Vec::new());

        for c in line.chars() {
            trees.get_mut(i).unwrap().push(c as u8 - b'0');
        }
    }

    for i in 1..trees.len() - 1 {
        for j in 1..trees.get(i).unwrap().len() - 1 {
            let tree = trees.get(i).unwrap().get(j).unwrap();
            let mut visible = true;

            // Top
            for k in 0..i {
                if trees.get(k).unwrap().get(j).unwrap() >= tree {
                    visible = false;
                }
            }

            if visible {
                visible_tree_count += 1;
                continue;
            }

            visible = true;

            // Bottom
            for l in i + 1..trees.len() {
                if trees.get(l).unwrap().get(j).unwrap() >= tree {
                    visible = false;
                }
            }

            if visible {
                visible_tree_count += 1;
                continue;
            }

            visible = true;

            // Left
            for m in 0..j {
                if trees.get(i).unwrap().get(m).unwrap() >= tree {
                    visible = false;
                }
            }

            if visible {
                visible_tree_count += 1;
                continue;
            }

            visible = true;

            // Right
            for n in j + 1..trees.get(i).unwrap().len() {
                if trees.get(i).unwrap().get(n).unwrap() >= tree {
                    visible = false;
                }
            }

            if visible {
                visible_tree_count += 1;
            }
        }
    }

    visible_tree_count += trees.len() as i32 * 2 + (trees.first().unwrap().len() as i32 - 2) * 2;

    visible_tree_count
}

fn part2() -> i32 {
    let input = include_str!("../input");
    let mut trees = Vec::<Vec<u8>>::new();

    let mut max_scenic_score = 0;

    for (i, line) in input.lines().enumerate() {
        trees.push(Vec::new());

        for c in line.chars() {
            trees.get_mut(i).unwrap().push(c as u8 - b'0');
        }
    }

    for i in 1..trees.len() - 1 {
        for j in 1..trees.get(0).unwrap().len() - 1 {
            let tree = trees.get(i).unwrap().get(j).unwrap();
            let mut top = 0;
            let mut bottom = 0;
            let mut left = 0;
            let mut right = 0;

            // Top
            for k in (0..=i - 1).rev() {
                top += 1;

                if trees.get(k).unwrap().get(j).unwrap() >= tree {
                    break;
                }
            }

            // Bottom
            for l in (i + 1)..trees.len() {
                bottom += 1;

                if trees.get(l).unwrap().get(j).unwrap() >= tree {
                    break;
                }
            }

            // Left
            for m in (0..=j - 1).rev() {
                left += 1;

                if trees.get(i).unwrap().get(m).unwrap() >= tree {
                    break;
                }
            }

            // Right
            for n in (j + 1)..trees.get(i).unwrap().len() {
                right += 1;

                if trees.get(i).unwrap().get(n).unwrap() >= tree {
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

// https://adventofcode.com/2022/day/5
#![feature(test)]
extern crate test;

fn main() {
    part1();
    part2();
}

#[inline(always)]
fn part1() -> String {
    let (mut stacks, rearrangements) = get_stacks_and_arrangements();
    let mut crates = String::new();

    for (num_crates, from_stack, to_stack) in rearrangements {
        for _ in 0..num_crates {
            let c = stacks
                .get_mut((from_stack - 1) as usize)
                .unwrap()
                .pop()
                .unwrap();
            stacks.get_mut((to_stack - 1) as usize).unwrap().push(c);
        }
    }

    for stack in &mut stacks {
        if let Some(c) = stack.pop() {
            crates += &c.to_string();
        }
    }

    crates
}

#[inline(always)]
fn part2() -> String {
    let (mut stacks, rearrangements) = get_stacks_and_arrangements();
    let mut crates = String::new();

    for (num_crates, from_stack, to_stack) in rearrangements {
        let stack = stacks.get_mut((from_stack - 1) as usize).unwrap();
        let crates = stack
            .drain(stack.len() - num_crates as usize..)
            .collect::<Vec<_>>();

        stacks
            .get_mut((to_stack - 1) as usize)
            .unwrap()
            .extend(crates);
    }

    for stack in &mut stacks {
        if let Some(c) = stack.pop() {
            crates += &c.to_string();
        }
    }

    crates
}

#[allow(clippy::type_complexity)]
#[inline(always)]
fn get_stacks_and_arrangements() -> (Vec<Vec<char>>, Vec<(u8, u8, u8)>) {
    let input = include_bytes!("../input");
    let mut input_index = 0;

    // Stacks
    let mut stacks = Vec::<Vec<char>>::new();
    let mut line_index = 0;

    loop {
        if input[input_index] == b'\n' {
            line_index = 0;

            if input[input_index + 1] == b'\n' {
                break;
            }
        }

        if input_index % 4 != 1 || input[input_index] < b'A' {
            // Skip everything which is not a letter
            input_index += 1;
            line_index += 1;
            continue;
        }

        let stack_index = line_index / 4;
        let stack = match stacks.get_mut(stack_index) {
            Some(stack) => stack,
            None => {
                if stack_index >= stacks.len() {
                    for _ in 0..(stack_index + 1 - stacks.len()) {
                        let stack = Vec::<char>::new();
                        stacks.push(stack);
                    }
                }

                stacks.get_mut(stack_index).unwrap()
            }
        };

        stack.insert(0, input[input_index] as char);
        input_index += 1;
        line_index += 1;
    }

    // Skip "\n\n"
    input_index += 2;

    // Rearrangements
    let mut rearrangements = Vec::<(u8, u8, u8)>::new();

    while input_index < input.len() {
        let mut num_crates;
        let mut from_stack;
        let mut to_stack;

        // Skip "move "
        input_index += 5;

        num_crates = input[input_index] - b'0';
        input_index += 1;

        if input[input_index] > b' ' {
            num_crates = num_crates * 10 + input[input_index] - b'0';
            input_index += 1;
        }

        // Skip " from "
        input_index += 6;

        from_stack = input[input_index] - b'0';
        input_index += 1;

        if input[input_index] > b' ' {
            from_stack = from_stack * 10 + input[input_index] - b'0';
            input_index += 1;
        }

        // Skip " to "
        input_index += 4;

        to_stack = input[input_index] - b'0';
        input_index += 1;

        if input_index < input.len() && input[input_index] > b' ' {
            to_stack = to_stack * 10 + input[input_index] - b'0';
            input_index += 1;
        }

        rearrangements.push((num_crates, from_stack, to_stack));

        // Skip a potential "\n"
        input_index += 1;
    }

    (stacks, rearrangements)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "BWNCQRMDB");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "NHWZCBNBF");
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

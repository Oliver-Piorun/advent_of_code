#![feature(drain_filter)]
use std::{collections::HashSet, fs, io};

#[derive(Default, Clone, Debug)]
struct Board {
    pub numbers: [[BoardNumber; 5]; 5],
}

#[derive(Default, Clone, Debug)]
struct BoardNumber {
    pub number: u8,
    pub marked: bool,
}

fn main() -> io::Result<()> {
    // Read the input
    let input = fs::read_to_string("input")?;

    // Split at two line breaks to get the sections of the input
    let sections = input.split("\r\n\r\n").collect::<Vec<_>>();

    // Get the drawn numbers (first section)
    let drawn_numbers = sections[0]
        .split(',')
        .map(|number_str| number_str.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // Get the boards (remaining sections)
    let mut boards = sections
        .iter()
        .skip(1)
        .map(|section| {
            // Split at whitespace to get the numbers of the section
            let split = section
                .split_whitespace()
                .map(|number_str| number_str.parse::<u8>().unwrap())
                .collect::<Vec<_>>();

            let mut board = Board::default();

            // Iterate over each row and each column
            for (row_index, chunk) in split.chunks_exact(5).enumerate() {
                chunk
                    .iter()
                    .enumerate()
                    .for_each(|(column_index, &number)| {
                        // Populate the board
                        board.numbers[column_index][row_index] = BoardNumber {
                            number,
                            marked: false,
                        }
                    })
            }

            board
        })
        .collect::<Vec<_>>();

    part1(&drawn_numbers, &mut boards.clone());
    part2(&drawn_numbers, &mut boards);

    Ok(())
}

fn part1(drawn_numbers: &[u8], boards: &mut [Board]) {
    // Iterate over each drawn number
    for drawn_number in drawn_numbers {
        // Iterate over each board
        for board in boards.iter_mut() {
            // Mark the drawn number and check if the board just won
            if !mark_and_check_for_win(*drawn_number, board) {
                // Board did not win
                continue;
            }

            // Board won

            // Get the sum of unmarked numbers
            let sum_of_unmarked_numbers = get_sum_of_ummarked_numbers(board);

            // Calculate the score
            let score = sum_of_unmarked_numbers * *drawn_number as u32;

            println!("part1: {score}");
            return;
        }
    }

    panic!("We should never land here!");
}

fn part2(drawn_numbers: &[u8], boards: &mut Vec<Board>) {
    let mut last_score = 0;

    // Iterate over each drawn number
    for drawn_number in drawn_numbers {
        // Iterate over each board and keep the ones which did not win yet
        *boards = boards
            .drain_filter(|board| {
                // Mark the drawn number and check if the board just won
                if !mark_and_check_for_win(*drawn_number, board) {
                    // Board did not won, so keep it
                    return true;
                }

                // Get the sum of unmarked numbers
                let sum_of_unmarked_numbers = get_sum_of_ummarked_numbers(board);

                // Calculate the score
                last_score = sum_of_unmarked_numbers * *drawn_number as u32;

                // Board did win, so do not keep it
                false
            })
            .collect::<Vec<_>>();

        // Check if there are boards which did not win yet
        if boards.is_empty() {
            break;
        }
    }

    println!("part2: {last_score}");
}

fn mark_and_check_for_win(drawn_number: u8, board: &mut Board) -> bool {
    // Row indices 0 to 4 are mapped to 0 to 4
    // Column indices 0 to 4 are mapped to 5 to 9
    let mut winning_indices = HashSet::<u8>::from_iter(0..10);

    for y in 0..5 {
        for x in 0..5 {
            let mut board_number = &mut board.numbers[x][y];

            if board_number.number == drawn_number {
                board_number.marked = true;
            } else if !board_number.marked {
                // Remove the row and column (indices) that contain an unmarked number
                winning_indices.remove(&(x as u8));
                winning_indices.remove(&((y + 5) as u8));
            }
        }
    }

    !winning_indices.is_empty()
}

fn get_sum_of_ummarked_numbers(board: &Board) -> u32 {
    // Iterate over each number and count the unmarked ones
    board.numbers.iter().flatten().fold(0, |sum, board_number| {
        if !board_number.marked {
            sum + board_number.number as u32
        } else {
            sum
        }
    })
}

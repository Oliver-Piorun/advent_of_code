use std::io::{self, BufReader};
use std::{fs::File, io::BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut highest_seat_id = 0;

    for line in lines {
        let mut row_lower_boundary: u8 = 0b0000_0000; // 0
        let mut row_upper_boundary: u8 = 0b0111_1111; // 127
        let mut column_lower_boundary: u8 = 0b0000_0000; // 0
        let mut column_upper_boundary: u8 = 0b0000_0111; // 7

        for (i, character) in line.chars().enumerate() {
            if character == 'F' {
                // Take lower half, update row upper boundary

                // Unset the bit
                row_upper_boundary &= !(1_u8 << (7 - i - 1));
            } else if character == 'B' {
                // Take upper half update row lower boundary

                // Set the bit
                row_lower_boundary |= 1_u8 << (7 - i - 1);
            } else if character == 'L' {
                // Take lower half, update column upper boundary

                // Unset the bit
                column_upper_boundary &= !(1_u8 << (7 + 3 - i - 1));
            } else if character == 'R' {
                // Take upper half, update column lower boundary

                // Set the bit
                column_lower_boundary |= 1_u8 << (7 + 3 - i - 1);
            }
        }

        assert_eq!(row_lower_boundary, row_upper_boundary);
        assert_eq!(column_lower_boundary, column_upper_boundary);

        let seat_id = row_lower_boundary as u32 * 8 + column_lower_boundary as u32;

        if highest_seat_id < seat_id {
            highest_seat_id = seat_id;
        }
    }

    println!("{highest_seat_id}");

    Ok(())
}

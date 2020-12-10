use std::{error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let mut numbers: Vec<u8> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<u8>().unwrap())
        .collect();
    numbers.sort_unstable();

    let mut num_arrangements = vec![0u64; numbers[numbers.len() - 1] as usize + 1];

    for &i in numbers.iter() {
        match i {
            0 => (),
            1 => num_arrangements[i as usize] = 1, // (1)
            2 => num_arrangements[i as usize] = 2, // (1, 2), (2)
            3 => num_arrangements[i as usize] = 4, // (1, 2, 3), (2, 3), (3), (1, 3)
            _ => {
                // Sum the number of arrangements of the previous three numbers
                // because that's exactly the number of arrangements for the current number
                num_arrangements[i as usize] += num_arrangements[i as usize - 3]
                    + num_arrangements[i as usize - 2]
                    + num_arrangements[i as usize - 1]
            }
        }
    }

    println!("{}", num_arrangements.last().unwrap());

    Ok(())
}

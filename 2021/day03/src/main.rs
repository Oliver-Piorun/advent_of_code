use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    str::FromStr,
};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input")?;

    // Read all lines
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    part1(&lines);
    part2(&lines);

    Ok(())
}

fn part1(lines: &[String]) {
    let mut num_unset_bits = 0;
    let mut num_set_bits = 0;

    let mut gamma_rate = 0;

    for bit_index in 0..=11 {
        // Take a look at each line
        for line in lines {
            let bit = line.chars().nth(bit_index).unwrap();

            if bit == '0' {
                num_unset_bits += 1;
            } else if bit == '1' {
                num_set_bits += 1;
            }
        }

        if num_set_bits > num_unset_bits {
            gamma_rate |= 1 << 11 - bit_index;
        }

        num_unset_bits = 0;
        num_set_bits = 0;
    }

    let epsilon_rate = !gamma_rate & 0b00000000000000000000111111111111;

    println!("{gamma_rate}");
    println!("{epsilon_rate}");

    println!("part1: {}", gamma_rate * epsilon_rate);
}

fn part2(lines: &[String]) {
    let mut oxygen_vec: Vec<String> = lines.to_vec();
    let mut co2_vec: Vec<String> = lines.to_vec();

    let mut bit_index = 0;

    while oxygen_vec.len() != 1 {
        println!("oxy len {}", oxygen_vec.len());
        oxygen_vec = oxygen(&oxygen_vec, bit_index);
        bit_index += 1;
    }

    bit_index = 0;

    while co2_vec.len() != 1 {
        println!("co2 len {}", co2_vec.len());
        co2_vec = co2(&co2_vec, bit_index);
        bit_index += 1;
    }

    println!("oxy {}", oxygen_vec.len());
    println!("co2 {}", co2_vec.len());

    println!("part2: {}", oxygen_vec.first().unwrap());
    println!("part2: {}", co2_vec.first().unwrap());
}

fn oxygen(lines: &[String], bit_index: usize) -> Vec<String> {
    let mut num_unset_bits = 0;
    let mut num_set_bits = 0;

    for line in lines {
        let bit = line.chars().nth(bit_index).unwrap();

        if bit == '0' {
            num_unset_bits += 1;
        } else if bit == '1' {
            num_set_bits += 1;
        }
    }

    let mut c = '\0';

    if num_set_bits >= num_unset_bits {
        c = '1';
    } else {
        c = '0';
    }

    let mut v = lines.to_vec();

    v.retain(|line| line.chars().nth(bit_index).unwrap() == c);

    v
}

fn co2(lines: &[String], bit_index: usize) -> Vec<String> {
    let mut num_unset_bits = 0;
    let mut num_set_bits = 0;

    for line in lines {
        let bit = line.chars().nth(bit_index).unwrap();

        if bit == '0' {
            num_unset_bits += 1;
        } else if bit == '1' {
            num_set_bits += 1;
        }
    }

    let mut c = '\0';

    if num_set_bits >= num_unset_bits {
        c = '0';
    } else {
        c = '1';
    }

    let mut v = lines.to_vec();

    v.retain(|line| line.chars().nth(bit_index).unwrap() == c);

    v
}

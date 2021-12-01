use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let input = read_to_string("input")?;
    let passports: Vec<&str> = input.split("\r\n\r\n").collect();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut num_valid_passports = 0;

    for passport in passports {
        if required_fields
            .iter()
            .all(|required_field| passport.contains(required_field))
        {
            num_valid_passports += 1;
        }
    }

    println!("{num_valid_passports}");

    Ok(())
}

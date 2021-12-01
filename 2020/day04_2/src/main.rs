use regex::Regex;
use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input")?;
    let passports: Vec<&str> = input.split("\r\n\r\n").collect();

    let required_field_regexs = vec![
        Regex::new(r"\bbyr:(19[2-9]\d|200[0-2])\b")?,
        Regex::new(r"\biyr:20(1\d|20)\b")?,
        Regex::new(r"\beyr:20(2\d|30)\b")?,
        Regex::new(r"\bhgt:(((1[5-8]\d|19[0-3])cm)|((59|6\d|7[0-6])in))\b")?,
        Regex::new(r"\bhcl:#[0-9a-f]{6}\b")?,
        Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b")?,
        Regex::new(r"\bpid:\d{9}\b")?,
    ];
    let mut num_valid_passports = 0;

    for passport in passports {
        if required_field_regexs
            .iter()
            .all(|required_field_regex| required_field_regex.is_match(passport))
        {
            num_valid_passports += 1;
        }
    }

    println!("{num_valid_passports}");

    Ok(())
}

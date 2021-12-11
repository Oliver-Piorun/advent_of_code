use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::BufReader,
};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut bags = HashMap::new();

    for line in lines {
        let line_split: Vec<String> = line
            .split(" contain ")
            .map(|substring| substring.to_string())
            .collect();

        let mut bag = line_split[0].to_string();
        bag.pop();

        let child_bags: HashSet<String> = line_split[1]
            .split(", ")
            .filter_map(|substring| {
                let substring = substring.replace("bags", "bag").replace('.', "");

                if substring != "no other bag" {
                    let bag_split: Vec<&str> = substring.split(' ').collect();
                    let bag = bag_split[1..].join(" ");

                    return Some(bag);
                }

                None
            })
            .collect();

        bags.insert(bag, child_bags);
    }

    let num_can_hold_shiny_gold_bag = bags.keys().fold(0, |accumulator, bag| {
        if can_hold_shiny_gold_bag(bag, &bags) {
            accumulator + 1
        } else {
            accumulator
        }
    });

    println!("{num_can_hold_shiny_gold_bag}");

    Ok(())
}

fn can_hold_shiny_gold_bag(bag: &str, bags: &HashMap<String, HashSet<String>>) -> bool {
    for child_bag in &bags[bag] {
        if child_bag == "shiny gold bag" {
            return true;
        }

        if can_hold_shiny_gold_bag(child_bag, bags) {
            return true;
        }
    }

    false
}

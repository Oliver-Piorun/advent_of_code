use std::{collections::HashMap, error::Error, io::BufReader};
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

        let child_bags: HashMap<String, i32> = line_split[1]
            .split(", ")
            .filter_map(|substring| {
                let substring = substring.replace("bags", "bag").replace('.', "");

                if substring != "no other bag" {
                    let bag_split: Vec<&str> = substring.split(' ').collect();
                    let count = bag_split[0].parse::<i32>().unwrap();
                    let bag = bag_split[1..].join(" ");

                    return Some((bag, count));
                }

                None
            })
            .collect();

        bags.insert(bag, child_bags);
    }

    let num_child_bags = count_child_bags("shiny gold bag", &bags);

    println!("{num_child_bags}");

    Ok(())
}

fn count_child_bags(bag: &str, bags: &HashMap<String, HashMap<String, i32>>) -> i32 {
    bags[bag].iter().fold(0, |accumulator, (child_bag, count)| {
        accumulator + count + count * count_child_bags(child_bag, bags)
    })
}

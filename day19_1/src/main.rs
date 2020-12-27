use std::{collections::HashMap, error::Error, io::BufReader};
use std::{fs::File, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let empty_line_index = lines.iter().position(|line| line.is_empty()).unwrap();

    let rules = lines[0..empty_line_index]
        .iter()
        .map(|rule| {
            let split = rule.split(": ").collect::<Vec<&str>>();
            let rule_id = split[0].parse::<i32>().unwrap();
            let rule = split[1];

            (rule_id, rule)
        })
        .collect::<HashMap<i32, &str>>();
    let messages = lines[empty_line_index + 1..].to_vec();

    let resolved_rules = resolve_rules(rules[&0].to_string(), &rules);

    let mut num_matching_messages = 0;

    for message in messages {
        if resolved_rules.contains(&message) {
            num_matching_messages += 1;
        }
    }

    println!("{}", num_matching_messages);

    Ok(())
}

fn resolve_rules(mut rule: String, rules: &HashMap<i32, &str>) -> Vec<String> {
    if !rule.contains('\"') {
        // Examples:
        // 66 58 | 57 72
        // 117 99

        'main: loop {
            let copied_rule = rule.to_owned();

            // Get the groups
            let groups = copied_rule.split(" | ");

            for group in groups {
                // Get the first child rule id
                let child_rule_id_option = group
                    .split(' ')
                    .find(|&child_rule| child_rule.chars().next().unwrap().is_numeric())
                    .map(|child_rule| child_rule.parse::<i32>().unwrap());

                if let Some(child_rule_id) = child_rule_id_option {
                    // Resolve the child rules of the current child rule
                    let resolved_child_rules =
                        resolve_rules(rules[&child_rule_id].to_string(), rules);

                    // Form new groups based on the resolved child rules
                    let mut new_groups = Vec::new();

                    for resolved_child_rule in resolved_child_rules {
                        new_groups.push(group.replacen(
                            &child_rule_id.to_string(),
                            &resolved_child_rule,
                            1,
                        ));
                    }

                    // Expand the rule by the new groups
                    rule = rule.replacen(group, &new_groups.join(" | "), 1);

                    // Continue, because we just updated the rule and may not be done yet
                    continue 'main;
                }
            }

            // Return the rule, because it does not contain any child rules anymore
            return rule
                .split(" | ")
                .map(|group| group.chars().filter(|c| !c.is_whitespace()).collect())
                .collect::<Vec<String>>();
        }
    } else {
        // Examples:
        // "a"
        // "b"

        return vec![rule.chars().nth(1).unwrap().to_string()];
    }
}

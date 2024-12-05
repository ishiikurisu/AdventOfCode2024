use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn read_rule(line: String) -> (u64, u64) {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    let parts = line.split("|");
    let mut i: u64 = 0;
    let mut j: u64;

    for part in parts {
        j = part.parse::<u64>().unwrap(); 
        if i % 2 == 0 {
            x = j;
        } else {
            y = j;
        }
        i += 1;
    }

    return (x, y);
}

fn read_updates(line: String) -> Vec<u64> {
    let mut outlet: Vec<u64> = vec![];
    let parts = line.split(",");

    for part in parts {
        outlet.push(part.parse::<u64>().unwrap());
    }

    return outlet;
}

fn evaluate_updates(updates: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> u64 {
    let mut result: u64 = 0;
    let limit = updates.len();
    let mut valid: bool = true;
    let mut current_value: u64;
    let mut compared_value: u64;

    for i in 1..limit {
        current_value = updates[i];
        for j in 0..i {
            compared_value = updates[j];
            match rules.get(&current_value) {
                Some(ruleset) => {
                    if ruleset.contains(&compared_value) {
                        valid = false;
                    }
                },
                None => {
                },
            }
        }
    }

    if valid {
        result = updates[limit / 2];
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut input_state: u64 = 0;
    let mut x: u64;
    let mut y: u64;
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut result: u64 = 0;
    let mut updates: Vec<u64>;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            if input_state == 0 {
                (x, y) = read_rule(line);
                rules.entry(x).or_insert_with(HashSet::new).insert(y);
            } else if input_state == 1 {
                updates = read_updates(line);
                result += evaluate_updates(&updates, &rules);
            }
        } else {
            input_state += 1;
        }
    }

    println!("{}", result);
}


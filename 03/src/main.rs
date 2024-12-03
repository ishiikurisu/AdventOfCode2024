use regex::Regex;
use std::io::{self, BufRead};

fn capture(haystack: &String) -> Vec<String> {
    let re = Regex::new(r"mul\(\d\d?\d?,\d\d?\d?\)").unwrap();
    return re.find_iter(haystack).map(|m| m.as_str().to_string()).collect();
}

fn apply_mul(inlet: &String) -> i32 {
    let numbers_part = &inlet[4..(inlet.len()-1)];
    let raw_numbers = numbers_part.split(",");
    let mut result: i32 = 1;

    for raw_number in raw_numbers {
        result *= raw_number.parse::<i32>().unwrap();
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut matches: Vec<String>;
    let mut result: i32 = 0;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            matches = capture(&line);
            for m in matches {
                result += apply_mul(&m);
            }
        }
    }

    println!("{}", result);
}




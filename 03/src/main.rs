use regex::Regex;
use std::io::{self, BufRead};

fn capture(haystack: &String) -> Vec<String> {
    let pattern = r"(mul\(\d\d?\d?,\d\d?\d?\))|(do\(\))|(don't\(\))";
    let re = Regex::new(pattern).unwrap();
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
    let mut enabled: bool = true;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            matches = capture(&line);
            for m in matches {
                if m == "do()" {
                    enabled = true;
                } else if m == "don't()" {
                    enabled = false;
                } else if enabled {
                    result += apply_mul(&m); 
                }
            }
        }
    }

    println!("{}", result);
}




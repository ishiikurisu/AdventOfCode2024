extern crate regex;
use std::io::{self, BufRead};
use regex::Regex;

fn capture(inlet: &String) -> Vec<String> {
    let re = Regex::new(r"mul\(\d\d?\d?,\d\d?\d?\)").unwrap();
    let mut outlet: Vec<String> = vec![];

    for (_, [captured]) in re.captures_iter(inlet).map(|c| c.extract()) {
        outlet.push(captured);
    }

    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut matches: Vec<String>;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            matches = capture(&line);
            println!("--- # matches");
            for m in matches {
                println!("- {}", m);
            }
        }
    }
}


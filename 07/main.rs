use std::io::{self, BufRead};

fn read(inlet: &String) -> (u64, Vec<u64>) {
    let mut target: u64 = 0;
    let mut params: Vec<u64> = vec![];
    let first_layer = inlet.split(": ");
    let mut raw_second_layer: String = "".to_string(); 
    let mut i: u64 = 0;

    for part in first_layer {
        if i % 2 == 0 {
            target = part.parse::<u64>().unwrap();
        } else {
            raw_second_layer = part.to_string();
        }
        i += 1;
    }

    let second_layer = raw_second_layer.split(" ");
    for part in second_layer {
        params.push(part.parse::<u64>().unwrap());
    }

    return (target, params);
}

fn concat(x: u64, y: u64) -> u64 {
    let s = format!("{}{}", x, y);
    return s.parse::<u64>().unwrap();
}

fn backtrack(target: &u64, params: &Vec<u64>, index: usize, limit: usize, acc: u64) -> bool {
    if index == limit {
        return acc == *target;
    }
    if index == 0 {
        return backtrack(target, params, index + 1, limit, params[index]);
    }
    return backtrack(target, params, index + 1, limit, acc + params[index])
        || backtrack(target, params, index + 1, limit, acc * params[index])
        || backtrack(target, params, index + 1, limit, concat(acc, params[index]));
}

fn evaluate(target: &u64, params: &Vec<u64>) -> bool {
    return backtrack(target, params, 0, params.len(), 0);
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut target: u64;
    let mut params: Vec<u64>;
    let mut result: u64 = 0;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            (target, params) = read(&line);
            if evaluate(&target, &params) {
                result += target;
            }
        }
    }

    println!("{}", result);
}


use std::io::{self, BufRead};

fn parse(inlet: &String) -> Vec<Option<u64>> {
    const RADIX: u32 = 10;
    let mut outlet: Vec<Option<u64>> = vec![];
    let mut block_size: u8;
    let mut is_filled: bool = true;
    let mut current_id: u64 = 0;

    for c in inlet.chars() {
        block_size = c.to_digit(RADIX).unwrap() as u8;

        for _ in 0..block_size {
            if is_filled {
                outlet.push(Some(current_id));
            } else {
                outlet.push(None);
            }
        }

        if is_filled {
            current_id += 1;
        }
        is_filled = !is_filled;
    }

    return outlet;
}

fn compact(inlet: &Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut outlet: Vec<Option<u64>> = vec![];
    let limit: usize = inlet.len();
    let mut j: usize = limit - 1;
    let mut flag: bool;

    for it in inlet {
        outlet.push(*it);
    }

    for i in 0..limit {
        match outlet[i] {
            Some(_id) => {
            },
            None => {
                flag = true;
                while (i < j) && (flag == true) {
                    match outlet[j] {
                        Some(_other_id) => {
                            (outlet[i], outlet[j]) = (outlet[j], outlet[i]);
                            flag = false;
                        },
                        None => {
                        },
                    }
                    j -= 1;
                }
            },
        }
    }

    return outlet;
}

fn checksum(inlet: &Vec<Option<u64>>) -> u64 {
    let mut outlet: u64 = 0;

    for (i, block) in inlet.into_iter().enumerate() {
        match block {
            Some(id) => outlet += (i as u64) * (*id),
            None => {
            },
        }
    }

    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String; 
    let mut memory: Vec<Option<u64>>;
    let mut result: u64 = 0;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            memory = parse(&line);
            memory = compact(&memory);
            result = checksum(&memory);
        }
    }

    println!("{}", result);
}


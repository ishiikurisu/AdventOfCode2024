use std::io::{self, BufRead};
use std::collections::HashMap;

fn compute_similarity(list_a: &Vec<i32>, list_b_count: &HashMap<i32, i32>) -> i32 {
    let mut similarity: i32 = 0;

    for a in list_a {
	similarity += a * list_b_count.get(&a).copied().unwrap_or(0);
    }

    return similarity;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut list_a: Vec<i32> = vec![];
    let mut list_b_count: HashMap<i32, i32> = HashMap::<i32, i32>::new();
    let mut which_list: i32 = 0;
    let mut list_part: i32;
    let mut old_value: i32;
    let result: i32;

    for raw_line in stdin.lock().lines() {
	line = raw_line.unwrap();
	if line.len() > 0 {
	    let parts = line.split_whitespace();
	    for part in parts.clone().into_iter() {
		list_part = part.parse::<i32>().unwrap();
		if which_list % 2 == 0 {
		    list_a.push(list_part);
		} else {
		    old_value = list_b_count.get(&list_part).copied().unwrap_or(0);
		    list_b_count.insert(list_part, 1 + old_value);
		}
		which_list += 1;
	    }
	}
    }

    result = compute_similarity(&list_a, &list_b_count);
    println!("{}", result);
}


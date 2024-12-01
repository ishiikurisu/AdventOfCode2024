use std::io::{self, BufRead};

fn compute_difference(list_a: &Vec<i32>, list_b: &Vec<i32>, length: usize) -> i32 {
    let mut result: i32 = 0;

    for i in 0..length {
	result += (list_a[i] - list_b[i]).abs();
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];
    let mut which_list: i32 = 0;
    let mut list_part: i32;
    let mut length: usize = 0;
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
		    list_b.push(list_part);
		}
		which_list += 1;
	    }
	    length += 1;
	}
    }

    list_a.sort();
    list_b.sort();

    result = compute_difference(&list_a, &list_b, length);
    println!("{}", result);
}


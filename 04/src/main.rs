use std::io::{self, BufRead};
use regex::Regex;

fn count_occurrences_in_one_direction(haystack: &String) -> i32 {
    let pattern = r"XMAS";
    let re = Regex::new(pattern).unwrap();
    return re.find_iter(haystack).collect::<Vec<_>>().len().try_into().unwrap();
}

fn count_occurrences(haystack: &String) -> i32 {
    return count_occurrences_in_one_direction(haystack)
         + count_occurrences_in_one_direction(&haystack.to_string().chars().rev().collect::<String>());
}

fn find_horizontal(puzzle: &Vec<String>, _width: i32, _height: i32) -> i32 {
    let mut result: i32 = 0;

    for line in puzzle {
        result += count_occurrences(&line);
    }

    return result;
}

fn find_vertical(puzzle: &Vec<String>, width: i32, _height: i32) -> i32 {
    let mut result: i32 = 0;
    let mut haystack: String;

    for i in 0..width {
        haystack = "".to_string();
        for line in puzzle {
            haystack.push(line.chars().nth(i as usize).unwrap());
        }
        result += count_occurrences(&haystack);
    }

    return result;
}

fn positive_diagonal_string(puzzle: &Vec<String>, width: i32, initial_width: i32, initial_height: i32) -> String {
    let mut outlet: String = "".to_string();
    let mut line: String;
    let mut x: i32 = initial_width; 
    let mut y: i32 = initial_height;

    while (y >= 0) && (x < width) {
        line = puzzle.into_iter().nth(y as usize).unwrap().to_string();
        outlet.push(line.chars().nth(x as usize).unwrap());
        x += 1;
        y -= 1;
    }

    return outlet;
}

fn find_positive_diagonal(puzzle: &Vec<String>, width: i32, height: i32) -> i32 {
    let mut result: i32 = 0;
    let mut line: String;

    for y in 1..height {
        line = positive_diagonal_string(puzzle, width, 0, y);
        result += count_occurrences(&line); 
    }
    for x in 1..width {
        line = positive_diagonal_string(puzzle, width, x, height - 1);
        result += count_occurrences(&line);
    }

    return result;
}

fn negative_diagonal_string(puzzle: &Vec<String>, width: i32, height: i32, initial_width: i32, initial_height: i32) -> String {
    let mut outlet: String = "".to_string();
    let mut line: String;
    let mut x: i32 = initial_width;
    let mut y: i32 = initial_height;

    while (x < width) && (y < height) {
        line = puzzle.into_iter().nth(y as usize).unwrap().to_string();
        outlet.push(line.chars().nth(x as usize).unwrap());
        x += 1;
        y += 1;
    }

    return outlet;
}

fn find_negative_diagonal(puzzle: &Vec<String>, width: i32, height: i32) -> i32 {
    let mut result: i32 = 0;
    let mut line: String;
    
    for x in (0..(width-1)).rev() {
        line = negative_diagonal_string(puzzle, width, height, x, 0);
        result += count_occurrences(&line);
    }
    for y in 1..(height-1) {
        line = negative_diagonal_string(puzzle, width, height, 0, y);
        result += count_occurrences(&line);
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut puzzle: Vec<String> = vec![];
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut first_line: bool = true;
    let mut result: i32 = 0;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            if first_line {
                width = line.len() as i32;
                first_line = false;
            }
            puzzle.push(line);
            height += 1;
        }
    }

    result += find_horizontal(&puzzle, width, height);
    result += find_vertical(&puzzle, width, height);
    result += find_positive_diagonal(&puzzle, width, height);
    result += find_negative_diagonal(&puzzle, width, height);

    println!("{}", result);
}




use std::io::{self, BufRead};

fn char_at(puzzle: &Vec<String>, x: i32, y: i32) -> char {
    let line = puzzle.into_iter().nth(y as usize).unwrap().to_string();
    return line.chars().nth(x as usize).unwrap();
}

fn is_xmas(puzzle: &Vec<String>, x: i32, y: i32) -> bool {
    let mut positive: String = "".to_string();
    let mut negative: String = "".to_string();

    positive.push(char_at(puzzle, x,     y));
    positive.push(char_at(puzzle, x + 1, y + 1));
    positive.push(char_at(puzzle, x + 2, y + 2));

    negative.push(char_at(puzzle, x + 2, y));
    negative.push(char_at(puzzle, x + 1, y + 1));
    negative.push(char_at(puzzle, x,     y + 2));

    return ((positive == "MAS") || (positive == "SAM")) && ((negative == "MAS") || (negative == "SAM"));
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

    for x in 0..(width-2) {
        for y in 0..(height-2) {
            if is_xmas(&puzzle, x, y) {
                result += 1;
            }
        }
    }

    println!("{}", result);
}




use std::io::{self, BufRead};

fn parse_input(line: String) -> Vec<i32> {
    let inlet = line.split_whitespace();
    let mut outlet: Vec<i32> = vec![];
    let mut out_s: i32;

    for in_s in inlet {
        out_s = in_s.parse::<i32>().unwrap();
        outlet.push(out_s);
    }

    return outlet;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut first_comparison: bool = true;
    let mut is_increasing_overall: bool = true;
    let mut is_increasing: bool;
    let mut difference: i32;
    let limit = report.len();

    for i in 1..limit {
        difference = report[i] - report[i - 1];
        is_increasing = difference > 0;

        if first_comparison {
            first_comparison = false;
            is_increasing_overall = is_increasing;
        }

        if is_increasing_overall != is_increasing {
            return false;
        }

        if difference.abs() < 1 || difference.abs() > 3 {
            return false;
        }
    }

    return true;
}

fn copy_without(inlet: &Vec<i32>, n: usize, size: usize) -> Vec<i32> {
    let mut outlet: Vec<i32> = vec![];

    for i in 0..size {
        if i != n {
            outlet.push(inlet[i]);
        }
    }

    return outlet;
}

fn is_any_report_safe(report: &Vec<i32>) -> bool {
    let mut inlet: Vec<i32>;
    let size = report.len();

    for n in 0..size {
        inlet = copy_without(report, n, size);
        if is_report_safe(&inlet) {
            return true;
        }
    }

    return false;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut result: i32 = 0;
    let mut report: Vec<i32>;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            report = parse_input(line);
            if is_report_safe(&report) {
                result += 1;
            } else if is_any_report_safe(&report) {
                result += 1;
            }
        }
    }

    println!("{}", result);
}


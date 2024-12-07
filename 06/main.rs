use std::io::{self, BufRead};

/*
 * up: 0
 * right: 1
 * down: 2
 * left: 3
 */

fn direction_to_delta(d: i64) -> (i64, i64) {
    match d {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    }
}

fn next_direction(d: i64) -> i64 {
    (d + 1) % 4
}

fn out_of_bounds(x: i64, y: i64, w: i64, h: i64) -> bool {
    (x < 0) || (x >= w) || (y < 0) || (y >= h)
}

fn is_wall(p: &Vec<bool>, w: i64, _h: i64, x: i64, y: i64) -> bool {
    p[(x + (y * w)) as usize]
}

// p: puzzle
// x0: initial x
// y0: initial y
// w: width
// h: height
// o: outlet
fn loops(p: &Vec<bool>, x0: i64, y0: i64, w: i64, h: i64) -> bool {
    let mut o0: Vec<bool> = vec![];
    let mut o1: Vec<bool> = vec![];
    let mut o2: Vec<bool> = vec![];
    let mut o3: Vec<bool> = vec![];
    let mut x: i64 = x0;
    let mut y: i64 = y0;
    let mut dx: i64;
    let mut dy: i64;
    let mut x1: i64;
    let mut y1: i64;
    let mut d: i64 = 0;
    let mut i: usize;

    // allocate memory
    for _x in 0..(h * w) {
        o0.push(false);
        o1.push(false);
        o2.push(false);
        o3.push(false);
    }

    // for each possible movement
    loop {
        // checking if guard has been here already in this state
        i = (x + (y * w)) as usize; 
        match d {
            0 => {
                if o0[i] {
                    return true;
                } else {
                    o0[i] = true;
                }
            },
            1 => {
                if o1[i] {
                    return true;
                } else {
                    o1[i] = true;
                }

            },
            2 => {
                if o2[i] {
                    return true;
                } else {
                    o2[i] = true;
                }

            },
            3 => {
                if o3[i] {
                    return true;
                } else {
                    o3[i] = true;
                }
            },
            _ => todo!(),
        }

        // determining next movement
        (dx, dy) = direction_to_delta(d); 
        (x1, y1) = (x + dx, y + dy);

        if out_of_bounds(x1, y1, w, h) {
            break;
        }

        if is_wall(p, w, h, x1, y1) {
            d = next_direction(d);
        } else {
            (x, y) = (x1, y1);
        }
    }

    // if he leaves the area, this never works
    return false;
}

fn evaluate(p: &Vec<bool>, x0: i64, y0: i64, w: i64, h: i64) -> i64 {
    let mut t: Vec<bool> = vec![];
    let limit: usize = p.len();
    let p0: usize = (x0 + (y0 * w)) as usize;
    let mut o: i64 = 0;

    // allocate memory
    for it in p {
        t.push(*it);
    }

    // try every possibility
    for i in 0..limit {
        if (!t[i]) && (i != p0) {
            t[i] = true;
            if loops(&t, x0, y0, w, h) {
                o += 1;
            }
            t[i] = false;
        }
    }

    return o;
}

fn find_initial_position(l: &String) -> i64 {
    for (i, c) in l.chars().enumerate() {
        if c == '^' {
            return i as i64;
        }
    }
    return -1;
}

fn main() {
    let stdin = io::stdin();
    let mut puzzle: Vec<bool> = vec![];
    let mut line: String;
    let mut width: i64 = 0;
    let mut height: i64 = 0;
    let mut x0: i64 = -1;
    let mut y0: i64 = -1;
    let mut x: i64;
    let mut first_line: bool = true;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            if first_line {
                width = line.len() as i64;
                first_line = false;
            }
            x = find_initial_position(&line);
            if x >= 0 {
                x0 = x;
                y0 = height;
            }
            height += 1;

            for c in line.chars() {
                puzzle.push(c == '#'); 
            }
        }
    }

    println!("{}", evaluate(&puzzle, x0, y0, width, height));
}


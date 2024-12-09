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

fn try_to_move(memory: &mut Vec<Option<u64>>, target_id: u64) {
    let mut i: usize;
    let mut block_size: usize;
    let mut block_index: usize;
    let mut is_inside_block: bool;
    let mut has_found_block: bool; 
    let mut room_index: usize;
    let mut room_size: usize;
    let mut has_found_room: bool;

    // finding block
    i = memory.len() - 1;
    has_found_block = false;
    is_inside_block = false;
    block_size = 0;
    block_index = 0;

    while (i > 0) && !has_found_block {
        match memory[i] {
            Some(id) => {
                if id == target_id {
                    is_inside_block = true;
                    block_size += 1;
                } else if is_inside_block {
                    block_index = i + 1;
                    has_found_block = true;
                }
            },
            None => {
                if is_inside_block {
                    block_index = i + 1;
                    has_found_block = true;
                }
            },
        }
        i -= 1;
    }

    // finding room
    i = 0;
    is_inside_block = false;
    has_found_room = false;
    room_index = 0;
    room_size = 0;

    while (i < block_index) && !has_found_room {
        match memory[i] {
            Some(_id) => {
                if is_inside_block {
                    if room_size >= block_size {
                        has_found_room = true;
                    }
                }
                is_inside_block = false;
            },
            None => {
                if !is_inside_block {
                    room_index = i;
                    room_size = 0;
                    is_inside_block = true;
                }
                room_size += 1;
            }
        }
        i += 1;
    }

    // moving block
    if has_found_room {
        i = 0;

        while i < block_size {
            (memory[block_index + i], memory[room_index + i]) =
                (memory[room_index + i], memory[block_index + i]);

            i += 1;
        }
    }
}

fn compact(inlet: &Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut outlet: Vec<Option<u64>> = vec![];
    let mut current_id: u64 = 0;

    for block in inlet {
        outlet.push(*block);
        match block {
            Some(id) => current_id = *id,
            None => {
            },
        }
    }

    while current_id > 0 {
        try_to_move(&mut outlet, current_id); 
        current_id -= 1;
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
    let mut result: u64;

    for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            memory = parse(&line);
            memory = compact(&memory);
            result = checksum(&memory);
            println!("{}", result);
        }
    }
}


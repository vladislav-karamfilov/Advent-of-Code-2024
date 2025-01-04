fn main() {
    solve_puzzle1();
}

// https://adventofcode.com/2024/day/25
fn solve_puzzle1() {
    let (pin_heights_of_locks, pin_heights_of_keys) = read_lock_pin_heights_and_key_pin_heights();

    let mut fits = 0;
    for lock_pin_heights in pin_heights_of_locks.iter() {
        for key_pin_heights in pin_heights_of_keys.iter() {
            if does_key_fit_lock(key_pin_heights, lock_pin_heights) {
                fits += 1;
            }
        }
    }

    println!("{fits}");
}

fn does_key_fit_lock(key_pin_heights: &Vec<u8>, lock_pin_heights: &Vec<u8>) -> bool {
    for i in 0..key_pin_heights.len() {
        if key_pin_heights[i] + lock_pin_heights[i] > 5 {
            return false;
        }
    }

    true
}

fn determine_pin_heights(key_or_lock_schematic: &[Vec<char>], is_key: bool) -> Vec<u8> {
    let mut result = Vec::with_capacity(key_or_lock_schematic[0].len());

    for col in 0..key_or_lock_schematic[0].len() {
        let mut height = 0;
        let mut row = if is_key {
            key_or_lock_schematic.len() - 1
        } else {
            0
        };

        while key_or_lock_schematic[row][col] == '#' {
            height += 1;

            row = if is_key { row - 1 } else { row + 1 };
        }

        result.push(height - 1);
    }

    result
}

fn read_lock_pin_heights_and_key_pin_heights() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut lock_pin_heights = vec![];
    let mut key_pin_heights = vec![];
    let mut is_reading_pin_heights = true;
    let mut key_or_lock_schematic: Vec<Vec<char>> = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            if is_reading_pin_heights {
                let is_key = key_or_lock_schematic[0][0] == '.';
                let pin_heights = determine_pin_heights(&key_or_lock_schematic, is_key);
                if is_key {
                    key_pin_heights.push(pin_heights);
                } else {
                    lock_pin_heights.push(pin_heights);
                }

                key_or_lock_schematic = vec![];
                is_reading_pin_heights = false;
                continue;
            }

            break;
        }

        key_or_lock_schematic.push(line.chars().collect());
        is_reading_pin_heights = true;
    }

    (lock_pin_heights, key_pin_heights)
}

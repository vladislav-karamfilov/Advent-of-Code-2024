use std::collections::{HashMap, HashSet};

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let (antennas_map, max_row, max_col) = read_antennas_map_and_max_row_and_max_col();

    let mut antinode_coords = HashSet::new();
    for same_freq_antenna_coords in antennas_map.values() {
        for i in 0..same_freq_antenna_coords.len() - 1 {
            for j in i + 1..same_freq_antenna_coords.len() {
                find_antinode_coords(
                    &same_freq_antenna_coords[i],
                    &same_freq_antenna_coords[j],
                    max_row,
                    max_col,
                    &mut antinode_coords,
                );
            }
        }
    }

    println!("{}", antinode_coords.len());
}

fn find_antinode_coords(
    antenna1_coords: &Coordinate2D,
    antenna2_coords: &Coordinate2D,
    max_row: i32,
    max_col: i32,
    antinode_coords: &mut HashSet<Coordinate2D>,
) {
    let row_diff = antenna2_coords.row - antenna1_coords.row;
    let col_diff = antenna2_coords.col - antenna1_coords.col;

    let gcd = calculate_greatest_common_divisor(row_diff.abs(), col_diff.abs());

    let step_row = row_diff / gcd;
    let step_col = col_diff / gcd;

    let first_antinode_row = antenna1_coords.row - step_row;
    let first_antinode_col = antenna1_coords.col - step_col;
    let second_antinode_row = antenna2_coords.row + step_row;
    let second_antinode_col = antenna2_coords.col + step_col;

    if first_antinode_row >= 0
        && first_antinode_row <= max_row
        && first_antinode_col >= 0
        && first_antinode_col <= max_col
    {
        antinode_coords.insert(Coordinate2D {
            row: first_antinode_row,
            col: first_antinode_col,
        });
    }

    if second_antinode_row >= 0
        && second_antinode_row <= max_row
        && second_antinode_col >= 0
        && second_antinode_col <= max_col
    {
        antinode_coords.insert(Coordinate2D {
            row: second_antinode_row,
            col: second_antinode_col,
        });
    }
}

fn calculate_greatest_common_divisor(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        calculate_greatest_common_divisor(b, a % b)
    }
}

fn read_antennas_map_and_max_row_and_max_col() -> (HashMap<char, Vec<Coordinate2D>>, i32, i32) {
    let mut antennas_map = HashMap::new();

    let mut row = 0;
    let mut max_col = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        max_col = trimmed_line.len() - 1;

        for (col, ch) in trimmed_line.chars().enumerate() {
            if ch != '.' {
                let antenna_coords = antennas_map.entry(ch).or_insert(vec![]);
                antenna_coords.push(Coordinate2D {
                    row,
                    col: col as i32,
                });
            }
        }

        row += 1;
    }

    (antennas_map, row - 1, max_col as i32)
}

#[derive(Eq, Hash, PartialEq)]
struct Coordinate2D {
    row: i32,
    col: i32,
}

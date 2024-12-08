use std::collections::{HashMap, HashSet};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let (antennas_map, max_row, max_col) = read_antennas_map_and_max_row_and_max_col();

    let mut antinode_coords = HashSet::new();
    for same_freq_antenna_coords in antennas_map.values() {
        for i in 0..same_freq_antenna_coords.len() - 1 {
            for j in i + 1..same_freq_antenna_coords.len() {
                antinode_coords.insert(same_freq_antenna_coords[i]);
                antinode_coords.insert(same_freq_antenna_coords[j]);

                find_antinode_coords(
                    same_freq_antenna_coords[i],
                    same_freq_antenna_coords[j],
                    max_row,
                    max_col,
                    &mut antinode_coords,
                    true,
                    true,
                    true,
                );
            }
        }
    }

    println!("{}", antinode_coords.len());
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let (antennas_map, max_row, max_col) = read_antennas_map_and_max_row_and_max_col();

    let mut antinode_coords = HashSet::new();
    for same_freq_antenna_coords in antennas_map.values() {
        for i in 0..same_freq_antenna_coords.len() - 1 {
            for j in i + 1..same_freq_antenna_coords.len() {
                find_antinode_coords(
                    same_freq_antenna_coords[i],
                    same_freq_antenna_coords[j],
                    max_row,
                    max_col,
                    &mut antinode_coords,
                    false,
                    true,
                    true,
                );
            }
        }
    }

    println!("{}", antinode_coords.len());
}

fn find_antinode_coords(
    start_coord: Coordinate2D,
    end_coord: Coordinate2D,
    max_row: i32,
    max_col: i32,
    antinode_coords: &mut HashSet<Coordinate2D>,
    recursive: bool,
    search_up: bool,
    search_down: bool,
) {
    let row_diff = end_coord.row - start_coord.row;
    let col_diff = end_coord.col - start_coord.col;

    let gcd = calculate_greatest_common_divisor(row_diff.abs(), col_diff.abs());

    let step_row = row_diff / gcd;
    let step_col = col_diff / gcd;

    let first_antinode_coord = Coordinate2D {
        row: start_coord.row - step_row,
        col: start_coord.col - step_col,
    };

    let is_first_antinode_in_map =
        search_up && is_coordinate_in_map(first_antinode_coord, max_row, max_col);

    if is_first_antinode_in_map {
        antinode_coords.insert(first_antinode_coord);
    }

    let second_antinode_coord = Coordinate2D {
        row: end_coord.row + step_row,
        col: end_coord.col + step_col,
    };

    let is_second_antinode_in_map =
        search_down && is_coordinate_in_map(second_antinode_coord, max_row, max_col);

    if is_second_antinode_in_map {
        antinode_coords.insert(second_antinode_coord);
    }

    if recursive {
        if is_first_antinode_in_map {
            find_antinode_coords(
                first_antinode_coord,
                start_coord,
                max_row,
                max_col,
                antinode_coords,
                true,
                true,
                false,
            );
        }

        if is_second_antinode_in_map {
            find_antinode_coords(
                end_coord,
                second_antinode_coord,
                max_row,
                max_col,
                antinode_coords,
                true,
                false,
                true,
            );
        }
    }
}

fn is_coordinate_in_map(coord: Coordinate2D, max_row: i32, max_col: i32) -> bool {
    coord.row >= 0 && coord.row <= max_row && coord.col >= 0 && coord.col <= max_col
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate2D {
    row: i32,
    col: i32,
}

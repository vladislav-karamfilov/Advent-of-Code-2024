use std::collections::HashMap;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let (warehouse_map, robot_movements) = read_warehouse_map_and_robot_movements();

    let mut resized_warehouse_map = resize_warehouse_map(&warehouse_map);

    execute_robot_movements(robot_movements, &mut resized_warehouse_map);

    let sum_of_coordinates = calculate_sum_of_box_gps_coordinates(&resized_warehouse_map);

    println!("{sum_of_coordinates}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let (mut warehouse_map, robot_movements) = read_warehouse_map_and_robot_movements();

    execute_robot_movements(robot_movements, &mut warehouse_map);

    let sum_of_coordinates = calculate_sum_of_box_gps_coordinates(&warehouse_map);

    println!("{sum_of_coordinates}");
}

fn execute_robot_movements(
    robot_movements: Vec<MoveDirection>,
    warehouse_map: &mut Vec<Vec<char>>,
) {
    let robot_row = warehouse_map.iter().position(|l| l.contains(&'@')).unwrap();
    let robot_col = warehouse_map[robot_row]
        .iter()
        .position(|tile| *tile == '@')
        .unwrap();

    warehouse_map[robot_row][robot_col] = '.';

    let mut robot_position = Position {
        row: robot_row,
        col: robot_col,
    };

    print_warehouse_map(&warehouse_map, robot_position);
    println!();

    for (i, move_direction) in robot_movements.iter().enumerate() {
        let next_robot_position =
            calculate_position_after_movement(robot_position, *move_direction, 1);

        if warehouse_map[next_robot_position.row][next_robot_position.col] == '#' {
            continue;
        }

        if warehouse_map[next_robot_position.row][next_robot_position.col] == 'O'
            && !try_move_box(next_robot_position, *move_direction, warehouse_map)
        {
            continue;
        }

        if (warehouse_map[next_robot_position.row][next_robot_position.col] == '['
            || warehouse_map[next_robot_position.row][next_robot_position.col] == ']')
            && !try_move_resized_box(next_robot_position, *move_direction, warehouse_map)
        {
            continue;
        }

        robot_position = next_robot_position;

        if i > 305 {
            print_warehouse_map(&warehouse_map, robot_position);
            println!();
        }
    }

    print_warehouse_map(&warehouse_map, robot_position);
}

fn try_move_box(
    box_position: Position,
    move_direction: MoveDirection,
    warehouse_map: &mut [Vec<char>],
) -> bool {
    let mut next_box_position = calculate_position_after_movement(box_position, move_direction, 1);

    while warehouse_map[next_box_position.row][next_box_position.col] == 'O' {
        next_box_position = calculate_position_after_movement(next_box_position, move_direction, 1);
    }

    if warehouse_map[next_box_position.row][next_box_position.col] == '.' {
        warehouse_map[next_box_position.row][next_box_position.col] = 'O';
        warehouse_map[box_position.row][box_position.col] = '.';

        return true;
    }

    false
}

fn try_move_resized_box(
    box_position: Position,
    move_direction: MoveDirection,
    warehouse_map: &mut [Vec<char>],
) -> bool {
    if move_direction == MoveDirection::Right || move_direction == MoveDirection::Left {
        try_move_resized_box_right_or_left(box_position, move_direction, warehouse_map)
    } else {
        try_move_resized_box_up_or_down(box_position, move_direction, warehouse_map)
    }
}

fn try_move_resized_box_up_or_down(
    box_position: Position,
    move_direction: MoveDirection,
    warehouse_map: &mut [Vec<char>],
) -> bool {
    let box_cols = match warehouse_map[box_position.row][box_position.col] {
        '[' => vec![box_position.col, box_position.col + 1],
        _ => vec![box_position.col - 1, box_position.col],
    };

    // Build the map of all box side containing cols per row while moving into the passed direction
    let mut box_cols_per_row = HashMap::new();
    box_cols_per_row.insert(box_position.row, box_cols);

    let mut row = box_position.row;
    loop {
        let next_row = if move_direction == MoveDirection::Up {
            row - 1
        } else {
            row + 1
        };

        for col in box_cols_per_row.get(&row).unwrap().clone() {
            let tile = warehouse_map[next_row][col];
            if tile == '[' {
                let box_cols_on_next_row = box_cols_per_row.entry(next_row).or_default();
                box_cols_on_next_row.push(col);
                box_cols_on_next_row.push(col + 1);
            } else if tile == ']' {
                let box_cols_on_next_row = box_cols_per_row.entry(next_row).or_default();
                box_cols_on_next_row.push(col - 1);
                box_cols_on_next_row.push(col);
            }
        }

        // If there are no box cols on next row we have reached map border or empty space for moving the boxes
        if box_cols_per_row.get(&next_row).is_none() {
            let should_move_boxes = box_cols_per_row
                .get(&row)
                .unwrap()
                .iter()
                .all(|col| warehouse_map[next_row][*col] == '.');

            if !should_move_boxes {
                // Not enough space for moving the boxes
                return false;
            }

            for (row, box_cols) in box_cols_per_row.iter() {
                let next_row = if move_direction == MoveDirection::Up {
                    row - 1
                } else {
                    row + 1
                };

                for col in box_cols {
                    warehouse_map[next_row][*col] = warehouse_map[*row][*col];
                }

                // Clean up on the left and/or right if needed
                let min_col = *box_cols.iter().min().unwrap();
                if warehouse_map[next_row][min_col - 1] == warehouse_map[next_row][min_col] {
                    warehouse_map[next_row][min_col - 1] = '.';
                }

                let max_col = *box_cols.iter().max().unwrap();
                if warehouse_map[next_row][max_col] == warehouse_map[next_row][max_col + 1] {
                    warehouse_map[next_row][max_col + 1] = '.';
                }
            }

            // Clean up the start box position
            for col in box_cols_per_row.get(&box_position.row).unwrap() {
                warehouse_map[box_position.row][*col] = '.';
            }

            return true;
        }

        row = next_row;
    }
}

fn try_move_resized_box_right_or_left(
    box_position: Position,
    move_direction: MoveDirection,
    warehouse_map: &mut [Vec<char>],
) -> bool {
    let next_box_position = calculate_position_after_movement(box_position, move_direction, 2);

    if (warehouse_map[next_box_position.row][next_box_position.col] == '['
        || warehouse_map[next_box_position.row][next_box_position.col] == ']')
        && !try_move_resized_box_right_or_left(next_box_position, move_direction, warehouse_map)
    {
        return false;
    }

    if warehouse_map[next_box_position.row][next_box_position.col] == '.' {
        if move_direction == MoveDirection::Right {
            warehouse_map[next_box_position.row][next_box_position.col - 1] = '[';
            warehouse_map[next_box_position.row][next_box_position.col] = ']';

            warehouse_map[box_position.row][box_position.col - 1] = '.';
            warehouse_map[box_position.row][box_position.col] = '.';
        } else {
            warehouse_map[next_box_position.row][next_box_position.col] = '[';
            warehouse_map[next_box_position.row][next_box_position.col + 1] = ']';

            warehouse_map[box_position.row][box_position.col] = '.';
            warehouse_map[box_position.row][box_position.col + 1] = '.';
        }

        return true;
    }

    false
}

fn calculate_position_after_movement(
    current_position: Position,
    move_direction: MoveDirection,
    tile_width: u8,
) -> Position {
    match move_direction {
        MoveDirection::Up => Position {
            row: current_position.row - tile_width as usize,
            ..current_position
        },
        MoveDirection::Down => Position {
            row: current_position.row + tile_width as usize,
            ..current_position
        },
        MoveDirection::Left => Position {
            col: current_position.col - tile_width as usize,
            ..current_position
        },
        MoveDirection::Right => Position {
            col: current_position.col + tile_width as usize,
            ..current_position
        },
    }
}

fn calculate_sum_of_box_gps_coordinates(warehouse_map: &[Vec<char>]) -> usize {
    let mut result = 0;

    for (row, line) in warehouse_map.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            if *tile == 'O' || *tile == '[' {
                result += row * 100 + col;
            }
        }
    }

    result
}

fn resize_warehouse_map(warehouse_map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity(warehouse_map.len());

    for line in warehouse_map {
        let mut resized_line = Vec::with_capacity(2 * line.len());

        for tile in line {
            match tile {
                'O' => {
                    resized_line.push('[');
                    resized_line.push(']');
                }
                '@' => {
                    resized_line.push('@');
                    resized_line.push('.');
                }
                _ => {
                    resized_line.push(*tile);
                    resized_line.push(*tile);
                }
            }
        }

        result.push(resized_line);
    }

    result
}

fn read_warehouse_map_and_robot_movements() -> (Vec<Vec<char>>, Vec<MoveDirection>) {
    let mut warehouse_map = vec![];
    let mut robot_movements = vec![];
    let mut is_reading_warehouse_map = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_warehouse_map {
                is_reading_warehouse_map = false;
                continue;
            }

            break;
        }

        if is_reading_warehouse_map {
            warehouse_map.push(trimmed_line.chars().collect());
        } else {
            robot_movements.extend(trimmed_line.chars().map(|tile| match tile {
                '^' => MoveDirection::Up,
                'v' => MoveDirection::Down,
                '<' => MoveDirection::Left,
                '>' => MoveDirection::Right,
                _ => unreachable!(),
            }));
        }
    }

    (warehouse_map, robot_movements)
}

#[allow(dead_code)]
fn print_warehouse_map(warehouse_map: &[Vec<char>], robot_position: Position) {
    for (row, line) in warehouse_map.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            if row == robot_position.row && col == robot_position.col {
                print!("@");
            } else {
                print!("{tile}");
            }
        }

        println!();
    }
}

#[derive(Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

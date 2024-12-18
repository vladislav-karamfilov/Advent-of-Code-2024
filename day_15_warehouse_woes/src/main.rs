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

    for move_direction in robot_movements {
        let next_robot_position =
            calculate_position_after_movement(robot_position, move_direction, 1);

        if warehouse_map[next_robot_position.row][next_robot_position.col] == '#' {
            continue;
        }

        if warehouse_map[next_robot_position.row][next_robot_position.col] == 'O'
            && !try_move_box(next_robot_position, move_direction, warehouse_map)
        {
            continue;
        }

        if (warehouse_map[next_robot_position.row][next_robot_position.col] == '['
            || warehouse_map[next_robot_position.row][next_robot_position.col] == ']')
            && !try_move_resized_box(next_robot_position, move_direction, warehouse_map)
        {
            continue;
        }

        robot_position = next_robot_position;

        print_warehouse_map(&warehouse_map, robot_position);
        println!();
    }

    // print_warehouse_map(&warehouse_map, robot_position);
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
        return try_move_resized_box_to_right_or_left(box_position, move_direction, warehouse_map);
    }

    todo!()
}

fn try_move_resized_box_to_right_or_left(
    box_position: Position,
    move_direction: MoveDirection,
    warehouse_map: &mut [Vec<char>],
) -> bool {
    let next_box_position = calculate_position_after_movement(box_position, move_direction, 2);

    if warehouse_map[next_box_position.row][next_box_position.col] == '['
        || warehouse_map[next_box_position.row][next_box_position.col] == ']'
    {
        return try_move_resized_box_to_right_or_left(
            next_box_position,
            move_direction,
            warehouse_map,
        );
    }

    if warehouse_map[next_box_position.row][next_box_position.col] == '.' {
        if move_direction == MoveDirection::Right {
            warehouse_map[next_box_position.row][next_box_position.col] = ']';
            warehouse_map[next_box_position.row][next_box_position.col - 1] = '[';

            let tile = warehouse_map[box_position.row][box_position.col];

            warehouse_map[box_position.row][box_position.col] = match tile {
                '[' => ']',
                ']' => '[',
                _ => '.',
            };

            warehouse_map[box_position.row][box_position.col - 1] = tile;
            warehouse_map[box_position.row][box_position.col - 2] = '.';
        } else {
            warehouse_map[next_box_position.row][next_box_position.col] = '[';
            warehouse_map[next_box_position.row][next_box_position.col + 1] = ']';

            let tile = warehouse_map[box_position.row][box_position.col];

            warehouse_map[box_position.row][box_position.col] = match tile {
                '[' => ']',
                ']' => '[',
                _ => '.',
            };

            warehouse_map[box_position.row][box_position.col + 1] = tile;
            warehouse_map[box_position.row][box_position.col + 2] = '.';
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

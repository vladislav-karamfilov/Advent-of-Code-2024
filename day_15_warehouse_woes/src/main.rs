fn main() {
    let (mut warehouse_map, robot_movements) = read_warehouse_map_and_robot_movements();

    execute_robot_movements(robot_movements, &mut warehouse_map);

    // print_warehouse_map(&warehouse_map, robot_position);

    let mut result = 0;
    for (row, line) in warehouse_map.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == 'O' {
                result += row * 100 + col;
            }
        }
    }

    println!("{result}");
}

fn execute_robot_movements(
    robot_movements: Vec<MoveDirection>,
    warehouse_map: &mut Vec<Vec<char>>,
) {
    let robot_row = warehouse_map.iter().position(|l| l.contains(&'@')).unwrap();
    let robot_col = warehouse_map[robot_row]
        .iter()
        .position(|ch| *ch == '@')
        .unwrap();

    warehouse_map[robot_row][robot_col] = '.';

    let mut robot_position = Position {
        row: robot_row,
        col: robot_col,
    };

    for move_direction in robot_movements {
        let next_robot_position = calculate_position_after_movement(robot_position, move_direction);

        if warehouse_map[next_robot_position.row][next_robot_position.col] == '#' {
            continue;
        }

        if warehouse_map[next_robot_position.row][next_robot_position.col] == 'O'
            && !try_move_box(next_robot_position, move_direction, warehouse_map)
        {
            continue;
        }

        robot_position = next_robot_position;

        // print_warehouse_map(&warehouse_map, robot_position);
        // println!();
    }
}

fn try_move_box(
    box_position: Position,
    move_direction: MoveDirection,
    warehouse_map: &mut [Vec<char>],
) -> bool {
    let mut next_box_position = calculate_position_after_movement(box_position, move_direction);

    while warehouse_map[next_box_position.row][next_box_position.col] == 'O' {
        next_box_position = calculate_position_after_movement(next_box_position, move_direction);
    }

    if warehouse_map[next_box_position.row][next_box_position.col] == '.' {
        warehouse_map[next_box_position.row][next_box_position.col] = 'O';
        warehouse_map[box_position.row][box_position.col] = '.';

        return true;
    }

    false
}

fn calculate_position_after_movement(
    current_position: Position,
    move_direction: MoveDirection,
) -> Position {
    match move_direction {
        MoveDirection::Up => Position {
            row: current_position.row - 1,
            col: current_position.col,
        },
        MoveDirection::Down => Position {
            row: current_position.row + 1,
            col: current_position.col,
        },
        MoveDirection::Left => Position {
            row: current_position.row,
            col: current_position.col - 1,
        },
        MoveDirection::Right => Position {
            row: current_position.row,
            col: current_position.col + 1,
        },
    }
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
            robot_movements.extend(trimmed_line.chars().map(|ch| match ch {
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
        for (col, ch) in line.iter().enumerate() {
            if row == robot_position.row && col == robot_position.col {
                print!("@");
            } else {
                print!("{ch}");
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

#[derive(Clone, Copy)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

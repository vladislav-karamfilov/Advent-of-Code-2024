use std::collections::HashSet;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/6#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let mut map = read_map();

    let start_row = map.iter().position(|r| r.contains(&'^')).unwrap();
    let start_col = map[start_row].iter().position(|ch| *ch == '^').unwrap();
    let start_position = Position {
        row: start_row,
        col: start_col,
    };

    let visited_positions =
        find_visited_positions_on_the_way_out(start_position, Direction::Up, &map);

    let mut count = 0;
    for potential_obstacle_position in visited_positions.iter() {
        map[potential_obstacle_position.row][potential_obstacle_position.col] = '#';

        if detect_cycle_on_the_way_out(start_position, Direction::Up, &map) {
            count += 1;
        }

        map[potential_obstacle_position.row][potential_obstacle_position.col] = '.';
    }

    println!("{count}");
}

// https://adventofcode.com/2024/day/6
#[allow(dead_code)]
fn solve_puzzle1() {
    let map = read_map();

    let start_row = map.iter().position(|r| r.contains(&'^')).unwrap();
    let start_col = map[start_row].iter().position(|ch| *ch == '^').unwrap();
    let start_position = Position {
        row: start_row,
        col: start_col,
    };

    let visited_positions =
        find_visited_positions_on_the_way_out(start_position, Direction::Up, &map);

    println!("{}", visited_positions.len());
}

fn detect_cycle_on_the_way_out(
    start_position: Position,
    start_direction: Direction,
    map: &[Vec<char>],
) -> bool {
    let mut current_position = start_position;
    let mut current_direction = start_direction;

    let mut visited_positions_with_direction = HashSet::new();
    visited_positions_with_direction.insert((current_position, current_direction));

    while let Some((next_position, next_direction)) =
        get_next_position_and_direction(current_position, current_direction, &map)
    {
        if !visited_positions_with_direction.insert((next_position, next_direction)) {
            return true;
        }

        current_position = next_position;
        current_direction = next_direction;
    }

    false
}

fn find_visited_positions_on_the_way_out(
    start_position: Position,
    start_direction: Direction,
    map: &[Vec<char>],
) -> HashSet<Position> {
    let mut current_position = start_position;
    let mut current_direction = start_direction;

    let mut visited_positions = HashSet::new();
    visited_positions.insert(current_position);

    while let Some((next_position, next_direction)) =
        get_next_position_and_direction(current_position, current_direction, &map)
    {
        visited_positions.insert(next_position);

        current_position = next_position;
        current_direction = next_direction;
    }

    visited_positions
}

fn get_next_position_and_direction(
    current_position: Position,
    current_direction: Direction,
    map: &[Vec<char>],
) -> Option<(Position, Direction)> {
    match current_direction {
        Direction::Right => {
            if current_position.col == map[0].len() - 1 {
                return None;
            }

            let next_position = Position {
                row: current_position.row,
                col: current_position.col + 1,
            };

            if map[next_position.row][next_position.col] == '#' {
                return get_next_position_and_direction(
                    current_position,
                    get_turn_direction(current_direction),
                    map,
                );
            }

            Some((next_position, current_direction))
        }
        Direction::Left => {
            if current_position.col == 0 {
                return None;
            }

            let next_position = Position {
                row: current_position.row,
                col: current_position.col - 1,
            };

            if map[next_position.row][next_position.col] == '#' {
                return get_next_position_and_direction(
                    current_position,
                    get_turn_direction(current_direction),
                    map,
                );
            }

            Some((next_position, current_direction))
        }
        Direction::Up => {
            if current_position.row == 0 {
                return None;
            }

            let next_position = Position {
                row: current_position.row - 1,
                col: current_position.col,
            };

            if map[next_position.row][next_position.col] == '#' {
                return get_next_position_and_direction(
                    current_position,
                    get_turn_direction(current_direction),
                    map,
                );
            }

            Some((next_position, current_direction))
        }
        Direction::Down => {
            if current_position.row == map.len() - 1 {
                return None;
            }

            let next_position = Position {
                row: current_position.row + 1,
                col: current_position.col,
            };

            if map[next_position.row][next_position.col] == '#' {
                return get_next_position_and_direction(
                    current_position,
                    get_turn_direction(current_direction),
                    map,
                );
            }

            Some((next_position, current_direction))
        }
    }
}

fn get_turn_direction(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
    }
}

fn read_map() -> Vec<Vec<char>> {
    let mut result = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        result.push(trimmed_line.chars().collect());
    }

    result
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

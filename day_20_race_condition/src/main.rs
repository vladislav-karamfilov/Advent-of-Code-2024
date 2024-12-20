use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let racetrack_map = read_racetrack_map();

    let start_row = racetrack_map.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = racetrack_map[start_row]
        .iter()
        .position(|tile| *tile == 'S')
        .unwrap();

    let end_row = racetrack_map.iter().position(|l| l.contains(&'E')).unwrap();
    let end_col = racetrack_map[end_row]
        .iter()
        .position(|tile| *tile == 'E')
        .unwrap();

    let start = Position {
        row: start_row,
        col: start_col,
    };

    let end = Position {
        row: end_row,
        col: end_col,
    };

    let mut cheat_positions = [Position::default(), Position::default()];

    match calculate_min_seconds_to_end(&racetrack_map, start, end, &cheat_positions) {
        Some(min_picoseconds) => {
            let mut target_cheats_count = 0;

            // Try horizontal cheats
            for row in 1..racetrack_map.len() - 1 {
                for col in 1..racetrack_map[row].len() - 2 {
                    if racetrack_map[row][col] == '#' && racetrack_map[row][col + 1] == '#' {
                        cheat_positions[0].row = row;
                        cheat_positions[0].col = col;
                        cheat_positions[1].row = row;
                        cheat_positions[1].col = col + 1;

                        // print_map(&racetrack_map, &cheat_positions);

                        if let Some(new_min_picoseconds) = calculate_min_seconds_to_end(
                            &racetrack_map,
                            start,
                            end,
                            &cheat_positions,
                        ) {
                            if new_min_picoseconds < min_picoseconds
                                && min_picoseconds - new_min_picoseconds >= 2
                            {
                                println!("saved: {}", min_picoseconds - new_min_picoseconds);

                                target_cheats_count += 1;
                            }
                        }
                    }
                }
            }

            // Try vertical cheats
            for col in 1..racetrack_map[0].len() - 1 {
                for row in 1..racetrack_map.len() - 2 {
                    if racetrack_map[row][col] == '#' && racetrack_map[row + 1][col] == '#' {
                        cheat_positions[0].row = row;
                        cheat_positions[0].col = col;
                        cheat_positions[1].row = row + 1;
                        cheat_positions[1].col = col;

                        // print_map(&racetrack_map, &cheat_positions);

                        if let Some(new_min_picoseconds) = calculate_min_seconds_to_end(
                            &racetrack_map,
                            start,
                            end,
                            &cheat_positions,
                        ) {
                            if new_min_picoseconds < min_picoseconds
                                && min_picoseconds - new_min_picoseconds >= 2
                            {
                                println!("saved: {}", min_picoseconds - new_min_picoseconds);

                                target_cheats_count += 1;
                            }
                        }
                    }
                }
            }

            println!("{target_cheats_count}")
        }
        None => println!("No path to end"),
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>], cheat_positions: &[Position]) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if cheat_positions[0].row == row && cheat_positions[0].col == col {
                print!("1");
            } else if cheat_positions[1].row == row && cheat_positions[1].col == col {
                print!("2");
            } else {
                print!("{}", map[row][col]);
            }
        }

        println!();
    }

    println!();
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_seconds_to_end(
    racetrack_map: &[Vec<char>],
    start: Position,
    end: Position,
    cheat_positions: &[Position],
) -> Option<u32> {
    let mut states = PriorityQueue::with_capacity(4 * racetrack_map.len());

    let mut initial_states = [
        PathState {
            position: start,
            move_direction: MoveDirection::Down,
            picoseconds: 0,
            estimated_distance_to_end: 0,
        },
        PathState {
            position: start,
            move_direction: MoveDirection::Up,
            picoseconds: 0,
            estimated_distance_to_end: 0,
        },
        PathState {
            position: start,
            move_direction: MoveDirection::Left,
            picoseconds: 0,
            estimated_distance_to_end: 0,
        },
        PathState {
            position: start,
            move_direction: MoveDirection::Right,
            picoseconds: 0,
            estimated_distance_to_end: 0,
        },
    ];

    initial_states.iter_mut().for_each(|s| {
        let state_score = s.get_score();
        states.push(s.clone(), Reverse(state_score));
    });

    let mut visited = HashSet::with_capacity(2 * racetrack_map.len());

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            return Some(current_state.picoseconds);
        }

        if !visited.insert((current_state.position, current_state.move_direction)) {
            continue;
        }

        let next_states = calculate_next_states(
            &current_state,
            end,
            racetrack_map,
            &visited,
            cheat_positions,
        );

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    None
}

fn calculate_next_states(
    current_state: &PathState,
    end: Position,
    racetrack_map: &[Vec<char>],
    visited: &HashSet<(Position, MoveDirection)>,
    cheat_positions: &[Position],
) -> Vec<PathState> {
    let mut result = Vec::with_capacity(4);

    let position = current_state.position;
    let new_picoseconds = current_state.picoseconds + 1;

    if position.row > 1 {
        let next_position = Position {
            row: position.row - 1,
            ..position
        };

        if (racetrack_map[next_position.row][next_position.col] != '#'
            || cheat_positions.contains(&next_position))
            && !visited.contains(&(next_position, MoveDirection::Up))
        {
            result.push(PathState {
                position: next_position,
                move_direction: MoveDirection::Up,
                picoseconds: new_picoseconds,
                estimated_distance_to_end: 0,
            });
        }
    }

    if position.row < racetrack_map.len() - 2 {
        let next_position = Position {
            row: position.row + 1,
            ..position
        };

        if (racetrack_map[next_position.row][next_position.col] != '#'
            || cheat_positions.contains(&next_position))
            && !visited.contains(&(next_position, MoveDirection::Down))
        {
            result.push(PathState {
                position: next_position,
                move_direction: MoveDirection::Up,
                picoseconds: new_picoseconds,
                estimated_distance_to_end: 0,
            });
        }
    }

    if position.col > 1 {
        let next_position = Position {
            col: position.col - 1,
            ..position
        };

        if (racetrack_map[next_position.row][next_position.col] != '#'
            || cheat_positions.contains(&next_position))
            && !visited.contains(&(next_position, MoveDirection::Left))
        {
            result.push(PathState {
                position: next_position,
                move_direction: MoveDirection::Left,
                picoseconds: new_picoseconds,
                estimated_distance_to_end: 0,
            });
        }
    }

    if position.col < racetrack_map.len() - 2 {
        let next_position = Position {
            col: position.col + 1,
            ..position
        };

        if (racetrack_map[next_position.row][next_position.col] != '#'
            || cheat_positions.contains(&next_position))
            && !visited.contains(&(next_position, MoveDirection::Right))
        {
            result.push(PathState {
                position: next_position,
                move_direction: MoveDirection::Right,
                picoseconds: new_picoseconds,
                estimated_distance_to_end: 0,
            });
        }
    }

    result
        .iter_mut()
        .for_each(|s| s.set_estimated_distance_to_end(end));

    result
}

fn read_racetrack_map() -> Vec<Vec<char>> {
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

#[derive(Hash, PartialEq, Eq, Clone)]
struct PathState {
    position: Position,
    move_direction: MoveDirection,
    picoseconds: u32,
    estimated_distance_to_end: u32,
}

impl PathState {
    fn get_score(&self) -> u32 {
        self.picoseconds + self.estimated_distance_to_end
    }

    fn set_estimated_distance_to_end(&mut self, end: Position) {
        self.estimated_distance_to_end =
            end.col.abs_diff(self.position.col) as u32 + end.row.abs_diff(self.position.row) as u32;
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

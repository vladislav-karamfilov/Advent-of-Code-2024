use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

fn main() {
    // solve_puzzle1(2);
    solve_puzzle1(100);
}

// https://adventofcode.com/2024/day/20
#[allow(dead_code)]
fn solve_puzzle1(min_saved_picoseconds: u32) {
    let mut racetrack_map = read_racetrack_map();

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

    match calculate_min_picoseconds_to_end(&racetrack_map, start, end) {
        Some(min_picoseconds) => {
            let mut target_cheats = 0;

            for row in 1..racetrack_map.len() - 1 {
                for col in 1..racetrack_map[row].len() - 1 {
                    if racetrack_map[row][col] != '#'
                        || are_all_neighbors_walls(row, col, &racetrack_map)
                    {
                        continue;
                    }

                    racetrack_map[row][col] = '.';

                    if let Some(new_min_picoseconds) =
                        calculate_min_picoseconds_to_end(&racetrack_map, start, end)
                    {
                        if new_min_picoseconds < min_picoseconds
                            && min_picoseconds - new_min_picoseconds >= min_saved_picoseconds
                        {
                            target_cheats += 1;
                        }
                    }

                    racetrack_map[row][col] = '#';
                }
            }

            println!("{target_cheats}");
        }
        None => println!("No path to end"),
    }
}

fn are_all_neighbors_walls(row: usize, col: usize, racetrack_map: &[Vec<char>]) -> bool {
    racetrack_map[row - 1][col] == '#'
        && racetrack_map[row + 1][col] == '#'
        && racetrack_map[row][col - 1] == '#'
        && racetrack_map[row][col + 1] == '#'
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_picoseconds_to_end(
    racetrack_map: &[Vec<char>],
    start: Position,
    end: Position,
) -> Option<u32> {
    let mut states = PriorityQueue::with_capacity(2 * racetrack_map.len());

    let initial_state = PathState {
        position: start,
        picoseconds: 0,
        estimated_distance_to_end: 0,
    };

    let initial_state_score = initial_state.get_score();
    states.push(initial_state, Reverse(initial_state_score));

    let mut visited = HashSet::with_capacity(2 * racetrack_map.len());

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            return Some(current_state.picoseconds);
        }

        if !visited.insert(current_state.position) {
            continue;
        }

        let next_states = calculate_next_states(&current_state, end, racetrack_map, &visited);

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
    visited: &HashSet<Position>,
) -> Vec<PathState> {
    let mut result = Vec::with_capacity(4);

    let position = current_state.position;
    let new_picoseconds = current_state.picoseconds + 1;

    if position.row > 1 {
        let next_position = Position {
            row: position.row - 1,
            ..position
        };

        if racetrack_map[next_position.row][next_position.col] != '#'
            && !visited.contains(&next_position)
        {
            result.push(PathState {
                position: next_position,
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

        if racetrack_map[next_position.row][next_position.col] != '#'
            && !visited.contains(&next_position)
        {
            result.push(PathState {
                position: next_position,
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

        if racetrack_map[next_position.row][next_position.col] != '#'
            && !visited.contains(&next_position)
        {
            result.push(PathState {
                position: next_position,
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

        if racetrack_map[next_position.row][next_position.col] != '#'
            && !visited.contains(&next_position)
        {
            result.push(PathState {
                position: next_position,
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

#[derive(Hash, PartialEq, Eq)]
struct PathState {
    position: Position,
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

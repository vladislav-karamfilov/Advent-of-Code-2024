use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use priority_queue::PriorityQueue;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let maze = read_maze();

    let start_row = maze.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = maze[start_row]
        .iter()
        .position(|tile| *tile == 'S')
        .unwrap();

    let end_row = maze.iter().position(|l| l.contains(&'E')).unwrap();
    let end_col = maze[end_row].iter().position(|tile| *tile == 'E').unwrap();

    let start = Position {
        row: start_row,
        col: start_col,
    };

    let end = Position {
        row: end_row,
        col: end_col,
    };

    match calculate_min_score_to_end(&maze, start, end) {
        Some(min_score) => {
            let unique_positions_on_paths_to_end =
                count_unique_positions_on_paths_to_end(&maze, start, end, min_score);

            println!("{unique_positions_on_paths_to_end}");
        }
        None => println!("None"),
    }
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let maze = read_maze();

    let start_row = maze.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = maze[start_row]
        .iter()
        .position(|tile| *tile == 'S')
        .unwrap();

    let end_row = maze.iter().position(|l| l.contains(&'E')).unwrap();
    let end_col = maze[end_row].iter().position(|tile| *tile == 'E').unwrap();

    let start = Position {
        row: start_row,
        col: start_col,
    };

    let end = Position {
        row: end_row,
        col: end_col,
    };

    let min_score = calculate_min_score_to_end(&maze, start, end);

    match min_score {
        Some(min_score) => println!("{min_score}"),
        None => println!("None"),
    }
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn count_unique_positions_on_paths_to_end(
    maze: &[Vec<char>],
    start: Position,
    end: Position,
    min_score: u32,
) -> usize {
    let mut states = PriorityQueue::with_capacity(5 * maze.len());

    let mut initial_state = PathState {
        position: start,
        cost: 0,
        estimated_distance_to_end: 0,
        move_direction: MoveDirection::East,
        previous_positions: vec![],
    };

    initial_state.set_estimated_distance_to_end(end);

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    let mut visited = HashMap::new();

    let mut unique_positions_on_paths_to_end = HashSet::with_capacity(2 * maze.len());
    unique_positions_on_paths_to_end.insert(start);
    unique_positions_on_paths_to_end.insert(end);

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            if current_state.cost == min_score {
                current_state.previous_positions.iter().for_each(|p| {
                    unique_positions_on_paths_to_end.insert(*p);
                });
            }

            continue;
        }

        if current_state.cost >= min_score {
            break;
        }

        if let Some(&visited_cost) =
            visited.get(&(current_state.position, current_state.move_direction))
        {
            if current_state.position != start && current_state.cost > visited_cost {
                continue;
            }
        }

        visited.insert(
            (current_state.position, current_state.move_direction),
            current_state.cost,
        );

        let next_states = calculate_next_states(&current_state, end, maze);

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    unique_positions_on_paths_to_end.len()
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_score_to_end(maze: &[Vec<char>], start: Position, end: Position) -> Option<u32> {
    let mut states = PriorityQueue::with_capacity(2 * maze.len());

    let mut initial_state = PathState {
        position: start,
        cost: 0,
        estimated_distance_to_end: 0,
        move_direction: MoveDirection::East,
        previous_positions: vec![],
    };

    initial_state.set_estimated_distance_to_end(end);

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    let mut visited = HashSet::with_capacity(2 * maze.len());

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            return Some(current_state.cost);
        }

        if !visited.insert((current_state.position, current_state.move_direction)) {
            continue;
        }

        let next_states = calculate_next_states(&current_state, end, maze);

        for next_state in next_states {
            if visited.contains(&(next_state.position, next_state.move_direction)) {
                continue;
            }

            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    None
}

fn calculate_next_states(
    current_state: &PathState,
    end: Position,
    maze: &[Vec<char>],
) -> Vec<PathState> {
    let mut result = Vec::with_capacity(3);

    let new_cost = current_state.cost + 1;
    let forward_position =
        calculate_forward_position(current_state.position, current_state.move_direction);

    if maze[forward_position.row][forward_position.col] != '#' {
        let mut previous_positions = current_state.previous_positions.clone();
        previous_positions.push(current_state.position);

        let mut new_state = PathState {
            position: forward_position,
            cost: new_cost,
            estimated_distance_to_end: 0,
            move_direction: current_state.move_direction,
            previous_positions,
        };

        new_state.set_estimated_distance_to_end(end);

        result.push(new_state);
    }

    let rotate_positions_and_directions = calculate_rotate_positions_and_directions(
        current_state.position,
        current_state.move_direction,
    );

    let new_cost = current_state.cost + 1_000;
    for (rotate_position, new_direction) in rotate_positions_and_directions {
        if maze[rotate_position.row][rotate_position.col] != '#' {
            result.push(PathState {
                position: current_state.position,
                cost: new_cost,
                estimated_distance_to_end: current_state.estimated_distance_to_end,
                move_direction: new_direction,
                previous_positions: current_state.previous_positions.clone(),
            });
        }
    }

    result
}

fn calculate_forward_position(
    current_position: Position,
    move_direction: MoveDirection,
) -> Position {
    match move_direction {
        MoveDirection::North => Position {
            row: current_position.row - 1,
            ..current_position
        },
        MoveDirection::South => Position {
            row: current_position.row + 1,
            ..current_position
        },
        MoveDirection::West => Position {
            col: current_position.col - 1,
            ..current_position
        },
        MoveDirection::East => Position {
            col: current_position.col + 1,
            ..current_position
        },
    }
}

fn calculate_rotate_positions_and_directions(
    current_position: Position,
    move_direction: MoveDirection,
) -> [(Position, MoveDirection); 2] {
    match move_direction {
        MoveDirection::North | MoveDirection::South => [
            (
                Position {
                    col: current_position.col - 1,
                    ..current_position
                },
                MoveDirection::West,
            ),
            (
                Position {
                    col: current_position.col + 1,
                    ..current_position
                },
                MoveDirection::East,
            ),
        ],
        MoveDirection::West | MoveDirection::East => [
            (
                Position {
                    row: current_position.row - 1,
                    ..current_position
                },
                MoveDirection::North,
            ),
            (
                Position {
                    row: current_position.row + 1,
                    ..current_position
                },
                MoveDirection::South,
            ),
        ],
    }
}

fn read_maze() -> Vec<Vec<char>> {
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
    cost: u32,
    estimated_distance_to_end: u32,
    move_direction: MoveDirection,
    previous_positions: Vec<Position>,
}

impl PathState {
    fn get_score(&self) -> u32 {
        self.cost + self.estimated_distance_to_end
    }

    fn set_estimated_distance_to_end(&mut self, end: Position) {
        self.estimated_distance_to_end =
            end.row.abs_diff(self.position.row) as u32 + end.col.abs_diff(self.position.col) as u32;
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum MoveDirection {
    North,
    South,
    West,
    East,
}

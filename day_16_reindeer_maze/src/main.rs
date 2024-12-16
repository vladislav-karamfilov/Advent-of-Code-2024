use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

fn main() {
    solve_puzzle1();
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
fn calculate_min_score_to_end(maze: &[Vec<char>], start: Position, end: Position) -> Option<u32> {
    let mut states = PriorityQueue::with_capacity(100);

    let mut initial_state = PathState {
        position: start,
        cost: 0,
        estimated_distance_to_end: 0,
        move_direction: MoveDirection::East,
    };

    initial_state.set_estimated_distance_to_end(end);

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    let mut seen = HashSet::with_capacity(100);

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            return Some(current_state.cost);
        }

        if !seen.insert((current_state.position, current_state.move_direction)) {
            continue;
        }

        let next_states = calculate_next_states(&current_state, end, maze, &seen);

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
    maze: &[Vec<char>],
    seen: &HashSet<(Position, MoveDirection)>,
) -> Vec<PathState> {
    let mut result = Vec::with_capacity(3);

    let new_cost = current_state.cost + 1;
    let forward_position = match current_state.move_direction {
        MoveDirection::North => Position {
            row: current_state.position.row - 1,
            ..current_state.position
        },
        MoveDirection::South => Position {
            row: current_state.position.row + 1,
            ..current_state.position
        },
        MoveDirection::West => Position {
            col: current_state.position.col - 1,
            ..current_state.position
        },
        MoveDirection::East => Position {
            col: current_state.position.col + 1,
            ..current_state.position
        },
    };

    if maze[forward_position.row][forward_position.col] != '#'
        && !seen.contains(&(forward_position, current_state.move_direction))
    {
        let mut new_state = PathState {
            position: forward_position,
            cost: new_cost,
            estimated_distance_to_end: 0,
            move_direction: current_state.move_direction,
        };

        new_state.set_estimated_distance_to_end(end);

        result.push(new_state);
    }

    let rotate_positions_and_directions = match current_state.move_direction {
        MoveDirection::North | MoveDirection::South => [
            (
                Position {
                    col: current_state.position.col - 1,
                    ..current_state.position
                },
                MoveDirection::West,
            ),
            (
                Position {
                    col: current_state.position.col + 1,
                    ..current_state.position
                },
                MoveDirection::East,
            ),
        ],
        MoveDirection::West | MoveDirection::East => [
            (
                Position {
                    row: current_state.position.row - 1,
                    ..current_state.position
                },
                MoveDirection::North,
            ),
            (
                Position {
                    row: current_state.position.row + 1,
                    ..current_state.position
                },
                MoveDirection::South,
            ),
        ],
    };

    let new_cost = current_state.cost + 1_000;
    for (rotate_position, new_direction) in rotate_positions_and_directions {
        if maze[rotate_position.row][rotate_position.col] != '#'
            && !seen.contains(&(current_state.position, new_direction))
        {
            result.push(PathState {
                position: current_state.position,
                cost: new_cost,
                estimated_distance_to_end: current_state.estimated_distance_to_end,
                move_direction: new_direction,
            });
        }
    }

    result
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

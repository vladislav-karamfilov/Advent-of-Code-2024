use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let door_codes = read_door_codes();

    for door_code in door_codes {
        todo!()
    }
}

fn calculate_complexity_of_door_code(door_code: &[char], button_presses: &[char]) -> u32 {
    let length = button_presses.len() as u32;

    let mut door_code_str = String::from_iter(door_code);
    door_code_str.pop();

    while door_code_str.starts_with('0') {
        door_code_str.remove(0);
    }

    let num = door_code_str.parse::<u32>().unwrap();

    length * num
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn find_min_button_presses_to_end(
    keypad: &[Vec<char>],
    start: Position,
    end: Position,
) -> Vec<char> {
    let mut states = PriorityQueue::with_capacity(2 * keypad.len());

    let initial_state = SequenceState {
        position: start,
        prev_positions: vec![],
        estimated_distance_to_end: 0,
    };

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            let mut button_presses = current_state
                .prev_positions
                .iter()
                .map(|p| keypad[p.row][p.col])
                .collect::<Vec<char>>();

            button_presses.push('A');

            return button_presses;
        }

        let next_states = calculate_next_states(&current_state, end, keypad);

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    unreachable!()
}

fn calculate_next_states(
    current_state: &SequenceState,
    end: Position,
    keypad: &[Vec<char>],
) -> Vec<SequenceState> {
    let mut result = Vec::with_capacity(4);

    let position = current_state.position;

    if position.row > 0 {
        let next_position = Position {
            row: position.row - 1,
            ..position
        };

        if keypad[next_position.row][next_position.col] != ' '
            && !current_state.prev_positions.contains(&next_position)
        {
            let mut new_prev_positions = current_state.prev_positions.clone();
            new_prev_positions.push(current_state.position);

            result.push(SequenceState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    if position.row < keypad.len() - 1 {
        let next_position = Position {
            row: position.row + 1,
            ..position
        };

        if keypad[next_position.row][next_position.col] != ' '
            && !current_state.prev_positions.contains(&next_position)
        {
            let mut new_prev_positions = current_state.prev_positions.clone();
            new_prev_positions.push(current_state.position);

            result.push(SequenceState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    if position.col > 0 {
        let next_position = Position {
            col: position.col - 1,
            ..position
        };

        if keypad[next_position.row][next_position.col] != ' '
            && !current_state.prev_positions.contains(&next_position)
        {
            let mut new_prev_positions = current_state.prev_positions.clone();
            new_prev_positions.push(current_state.position);

            result.push(SequenceState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    if position.col < keypad.len() - 1 {
        let next_position = Position {
            col: position.col + 1,
            ..position
        };

        if keypad[next_position.row][next_position.col] != ' '
            && !current_state.prev_positions.contains(&next_position)
        {
            let mut new_prev_positions = current_state.prev_positions.clone();
            new_prev_positions.push(current_state.position);

            result.push(SequenceState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    result
        .iter_mut()
        .for_each(|s| s.set_estimated_distance_to_end(end));

    result
}

fn read_door_codes() -> Vec<Vec<char>> {
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
struct SequenceState {
    position: Position,
    prev_positions: Vec<Position>,
    estimated_distance_to_end: u32,
}

impl SequenceState {
    fn get_score(&self) -> u32 {
        self.prev_positions.len() as u32 + self.estimated_distance_to_end
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

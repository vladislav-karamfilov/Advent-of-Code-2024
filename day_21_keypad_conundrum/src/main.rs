use std::cmp::Reverse;

use priority_queue::PriorityQueue;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let door_codes = read_door_codes();

    let directional_keypad = [[' ', '^', 'A'], ['<', 'v', '>']];
    let numeric_keypad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ];

    let mut sum = 0;
    for door_code in door_codes {
        let min_player_button_presses = calculate_min_player_button_presses_for_door_code(
            &door_code,
            &numeric_keypad,
            &directional_keypad,
        );

        let complexity = calculate_complexity_of_door_code(&door_code, &min_player_button_presses);
        println!("{} -> {complexity}", String::from_iter(&door_code));

        sum += complexity;
    }

    println!("{sum}");
}

fn calculate_min_player_button_presses_for_door_code(
    door_code: &Vec<char>,
    numeric_keypad: &[[char; 3]],
    directional_keypad: &[[char; 3]],
) -> Vec<char> {
    let numeric_keypad_start_position = Position { row: 3, col: 2 };
    let directional_keypad_start_position = Position { row: 0, col: 2 };

    let mut possibilites_of_robot2_button_presses = vec![];
    find_possibilities_for_next_button_presses(
        0,
        door_code,
        numeric_keypad,
        numeric_keypad_start_position,
        vec![],
        &mut possibilites_of_robot2_button_presses,
    );

    let mut possibilites_of_robot3_button_presses =
        Vec::with_capacity(10 * possibilites_of_robot2_button_presses.len());

    for robot2_button_presses in possibilites_of_robot2_button_presses {
        let mut current_possibilites_of_robot3_button_presses = vec![];
        find_possibilities_for_next_button_presses(
            0,
            &robot2_button_presses,
            directional_keypad,
            directional_keypad_start_position,
            vec![],
            &mut current_possibilites_of_robot3_button_presses,
        );

        possibilites_of_robot3_button_presses.extend(current_possibilites_of_robot3_button_presses);
    }

    let mut min_player_button_presses = vec![];
    for robot3_button_presses in possibilites_of_robot3_button_presses {
        let mut current_possibilites_of_player_button_presses = vec![];
        find_possibilities_for_next_button_presses(
            0,
            &robot3_button_presses,
            directional_keypad,
            directional_keypad_start_position,
            vec![],
            &mut current_possibilites_of_player_button_presses,
        );

        for player_button_presses in current_possibilites_of_player_button_presses {
            if min_player_button_presses.is_empty()
                || player_button_presses.len() < min_player_button_presses.len()
            {
                min_player_button_presses = player_button_presses;
            }
        }
    }

    min_player_button_presses
}

fn find_possibilities_for_next_button_presses(
    current_button_press_index: usize,
    button_presses: &[char],
    keypad: &[[char; 3]],
    start_position: Position,
    next_button_presses: Vec<char>,
    possibilities_for_next_button_presses: &mut Vec<Vec<char>>,
) {
    if current_button_press_index == button_presses.len() {
        possibilities_for_next_button_presses.push(next_button_presses);
        return;
    }

    let button = &button_presses[current_button_press_index];
    let button_position = find_button_position(button, keypad);

    if start_position == button_position {
        let mut new_next_button_presses = Vec::with_capacity(next_button_presses.len() + 1);
        new_next_button_presses.extend(next_button_presses);
        new_next_button_presses.push('A');

        find_possibilities_for_next_button_presses(
            current_button_press_index + 1,
            button_presses,
            keypad,
            button_position,
            new_next_button_presses,
            possibilities_for_next_button_presses,
        );

        return;
    }

    let keypad_paths = find_min_paths_to_end(keypad, start_position, button_position);
    for keypad_path in keypad_paths {
        let new_button_presses =
            transform_keypad_path_to_directional_button_presses(&keypad_path, start_position);

        let mut new_next_button_presses =
            Vec::with_capacity(&next_button_presses.len() + new_button_presses.len());

        new_next_button_presses.extend(&next_button_presses);
        new_next_button_presses.extend(&new_button_presses);

        find_possibilities_for_next_button_presses(
            current_button_press_index + 1,
            button_presses,
            keypad,
            button_position,
            new_next_button_presses,
            possibilities_for_next_button_presses,
        );
    }
}

fn find_button_position(button: &char, keypad: &[[char; 3]]) -> Position {
    let button_row = keypad.iter().position(|x| x.contains(button)).unwrap();
    let button_col = keypad[button_row].iter().position(|b| b == button).unwrap();

    Position {
        row: button_row,
        col: button_col,
    }
}

fn transform_keypad_path_to_directional_button_presses(
    keypad_path: &[Position],
    start_position: Position,
) -> Vec<char> {
    let mut result = Vec::with_capacity(keypad_path.len() + 1);

    let mut current_positon = start_position;

    for button_position in keypad_path {
        if button_position.row > current_positon.row {
            result.push('v');
        } else if button_position.row < current_positon.row {
            result.push('^');
        } else if button_position.col > current_positon.col {
            result.push('>');
        } else {
            result.push('<');
        }

        current_positon = *button_position;
    }

    result.push('A');

    result
}

fn calculate_complexity_of_door_code(door_code: &[char], button_presses: &[char]) -> u32 {
    let length = button_presses.len() as u32;

    let mut door_code_str = String::from_iter(&door_code[0..door_code.len() - 1]);

    while door_code_str.starts_with('0') {
        door_code_str.remove(0);
    }

    let num = door_code_str.parse::<u32>().unwrap();

    length * num
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn find_min_paths_to_end(
    keypad: &[[char; 3]],
    start: Position,
    end: Position,
) -> Vec<Vec<Position>> {
    let mut states = PriorityQueue::with_capacity(2 * keypad.len());

    let initial_state = PathState {
        position: start,
        prev_positions: vec![],
        estimated_distance_to_end: 0,
    };

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    let mut result: Vec<Vec<Position>> = vec![];

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            if !result.is_empty() && result[0].len() < current_state.prev_positions.len() {
                break;
            }

            let mut min_path = Vec::with_capacity(current_state.prev_positions.len());
            min_path.extend(current_state.prev_positions.iter().skip(1));
            min_path.push(current_state.position);

            result.push(min_path);
        }

        let next_states = calculate_next_states(&current_state, end, keypad);

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    result
}

fn calculate_next_states(
    current_state: &PathState,
    end: Position,
    keypad: &[[char; 3]],
) -> Vec<PathState> {
    let mut result = Vec::with_capacity(4);

    let position = current_state.position;

    // Right
    if position.col < keypad[0].len() - 1 {
        let next_position = Position {
            col: position.col + 1,
            ..position
        };

        if keypad[next_position.row][next_position.col] != ' '
            && !current_state.prev_positions.contains(&next_position)
        {
            let mut new_prev_positions = current_state.prev_positions.clone();
            new_prev_positions.push(current_state.position);

            result.push(PathState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    // Down
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

            result.push(PathState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    // Left
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

            result.push(PathState {
                position: next_position,
                prev_positions: new_prev_positions,
                estimated_distance_to_end: 0,
            });
        }
    }

    // Up
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

            result.push(PathState {
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
struct PathState {
    position: Position,
    prev_positions: Vec<Position>,
    estimated_distance_to_end: u32,
}

impl PathState {
    fn get_score(&self) -> u32 {
        self.prev_positions.len() as u32 + self.estimated_distance_to_end
    }

    fn set_estimated_distance_to_end(&mut self, end: Position) {
        self.estimated_distance_to_end =
            end.col.abs_diff(self.position.col) as u32 + end.row.abs_diff(self.position.row) as u32;
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

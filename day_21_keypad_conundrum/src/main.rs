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

    // A button positions for all robots
    let mut robot1_keypad_start_position = Position { row: 3, col: 2 };
    let mut robot2_keypad_start_position = Position { row: 0, col: 2 };
    let mut robot3_keypad_start_position = Position { row: 0, col: 2 };
    let mut player_keypad_start_position = Position { row: 0, col: 2 };

    let mut sum = 0;
    for door_code in door_codes {
        let (robot2_button_presses, end_position) = calculate_next_button_presses_and_keypad_end_position_after(
            &door_code,
            &numeric_keypad,
            robot1_keypad_start_position
        );

        robot1_keypad_start_position = end_position;

        println!("Robot 1: {} - {}", String::from_iter(&robot2_button_presses), robot2_button_presses.len());
        
        let (robot3_button_presses, end_position) = calculate_next_button_presses_and_keypad_end_position_after(
            &robot2_button_presses,
            &directional_keypad,
            robot2_keypad_start_position
        );

        robot2_keypad_start_position = end_position;

        println!("Robot 2: {} - {}", String::from_iter(&robot3_button_presses), robot3_button_presses.len());
        
        let (player_button_presses, end_position) = calculate_next_button_presses_and_keypad_end_position_after(
            &robot3_button_presses,
            &directional_keypad,
            robot3_keypad_start_position
        );

        robot3_keypad_start_position = end_position;

        println!("Robot 3: {} - {}", String::from_iter(&player_button_presses), player_button_presses.len());
        
        // let (player_button_presses, end_position) = calculate_next_button_presses_and_keypad_end_position_after(
        //     &robo,
        //     &numeric_keypad,
        //     player_keypad_start_position,
        // );

        // robot1_keypad_start_position = end_position;

        // player_button_presses = 

        // sum += calculate_complexity_of_door_code(&door_code, &player_button_presses);
    }

    println!("{sum}");
}

fn calculate_next_button_presses_and_keypad_end_position_after(
    button_presses: &[char],
    keypad: &[[char; 3]],
    start_position: Position,
) -> (Vec<char>, Position) {
    let mut next_button_presses = vec![];
    let mut current_position = start_position;

    for button in button_presses.iter() {
        let button_position = find_button_position(button, keypad);

        let keypad_path = find_min_path_to_end(keypad, current_position, button_position);

        let new_buttton_presses =
            transform_keypad_path_to_directional_button_presses(&keypad_path, current_position);

        next_button_presses.extend_from_slice(&new_buttton_presses);

        current_position = button_position;
    }

    (next_button_presses, current_position)
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
        if current_positon.row < button_position.row {
            result.push('v');
        } else if current_positon.row > button_position.row {
            result.push('^');
        } else {
            if current_positon.col < button_position.col {
                result.push('>');
            } else {
                result.push('<');
            }
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
fn find_min_path_to_end(keypad: &[[char; 3]], start: Position, end: Position) -> Vec<Position> {
    if start == end {
        return vec![];
    }

    let mut states = PriorityQueue::with_capacity(2 * keypad.len());

    let initial_state = PathState {
        position: start,
        prev_positions: vec![],
        estimated_distance_to_end: 0,
    };

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    while let Some((current_state, _)) = states.pop() {
        if current_state.position == end {
            let mut positions = Vec::with_capacity(current_state.prev_positions.len());

            for i in 1..current_state.prev_positions.len() {
                positions.push(current_state.prev_positions[i]);
            }

            positions.push(current_state.position);

            return positions;
        }

        let next_states = calculate_next_states(&current_state, end, keypad);

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    vec![]
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

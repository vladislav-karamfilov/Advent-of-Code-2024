use std::{cmp::Reverse, collections::HashSet, vec};

use priority_queue::PriorityQueue;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/13#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let button_move_vectors_and_prize_coords = read_button_move_vectors_and_prize_coords();

    let total_cost = button_move_vectors_and_prize_coords
        .iter()
        .map(|(a, b, p)| {
            let actual_prize_coord = Coordinate2D {
                x: p.x + 10_000_000_000_000,
                y: p.y + 10_000_000_000_000,
            };

            if let Some(min_cost) = calculate_min_cost_to_prize(*a, *b, actual_prize_coord) {
                min_cost
            } else {
                0
            }
        })
        .sum::<u64>();

    println!("{total_cost}");
}

// https://adventofcode.com/2024/day/13
#[allow(dead_code)]
fn solve_puzzle1() {
    let button_move_vectors_and_prize_coords = read_button_move_vectors_and_prize_coords();

    let total_cost = button_move_vectors_and_prize_coords
        .iter()
        .map(|(a, b, p)| {
            if let Some(min_cost) = calculate_min_cost_to_prize_naive(*a, *b, *p) {
                min_cost
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{total_cost}");
}

fn calculate_min_cost_to_prize(
    button_a_move_vector: Coordinate2D,
    button_b_move_vector: Coordinate2D,
    prize_coord: Coordinate2D,
) -> Option<u64> {
    // a_pushes * Ax + b_pushes * Bx = Px
    // a_pushes * Ay + b_pushes * By = Py
    // =>
    // a_pushes = (Px - b_pushes * Bx) / Ax
    // b_pushes = (Py - a_pushes * Ay) / By
    // =>
    // b_pushes = (Ax * Py - Ay * Px) / (By * Ax - Bx * Ay)

    let b_pushes = (button_a_move_vector.x * prize_coord.y
        - button_a_move_vector.y * prize_coord.x)
        / (button_b_move_vector.y * button_a_move_vector.x
            - button_b_move_vector.x * button_a_move_vector.y);

    let rem_b = (button_a_move_vector.x * prize_coord.y - button_a_move_vector.y * prize_coord.x)
        % (button_b_move_vector.y * button_a_move_vector.x
            - button_b_move_vector.x * button_a_move_vector.y);

    let a_pushes = (prize_coord.x - b_pushes * button_b_move_vector.x) / button_a_move_vector.x;
    let rem_a = (prize_coord.x - b_pushes * button_b_move_vector.x) % button_a_move_vector.x;

    if rem_b != 0 || rem_a != 0 {
        None
    } else {
        Some(3 * a_pushes as u64 + b_pushes as u64)
    }
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_cost_to_prize_naive(
    button_a_move_vector: Coordinate2D,
    button_b_move_vector: Coordinate2D,
    prize_coord: Coordinate2D,
) -> Option<u32> {
    let mut states = PriorityQueue::with_capacity(100);

    let mut initial_state = ClawMachineState {
        coord: Coordinate2D { x: 0, y: 0 },
        cost: 0,
        estimated_distance_to_end: 0,
        button_a_pushes: 0,
        button_b_pushes: 0,
    };

    initial_state.set_estimated_distance_to_end(prize_coord);

    let state_score = initial_state.get_score();
    states.push(initial_state, Reverse(state_score));

    let mut seen = HashSet::new();

    while let Some((current_state, _)) = states.pop() {
        if current_state.coord == prize_coord {
            return Some(current_state.cost);
        }

        seen.insert((current_state.coord, current_state.cost));

        let next_states = calculate_next_states(
            &current_state,
            button_a_move_vector,
            button_b_move_vector,
            prize_coord,
            &seen,
        );

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    None
}

fn calculate_next_states(
    current_state: &ClawMachineState,
    button_a_move_vector: Coordinate2D,
    button_b_move_vector: Coordinate2D,
    prize_coord: Coordinate2D,
    seen: &HashSet<(Coordinate2D, u32)>,
) -> Vec<ClawMachineState> {
    let mut result = Vec::with_capacity(2);

    if current_state.button_a_pushes < 100 {
        let coord_after_button_a_push = Coordinate2D {
            x: current_state.coord.x + button_a_move_vector.x,
            y: current_state.coord.y + button_a_move_vector.y,
        };

        let cost_after_button_a_push = current_state.cost + 3;

        if coord_after_button_a_push.x <= prize_coord.x
            && coord_after_button_a_push.y <= prize_coord.y
            && !seen.contains(&(coord_after_button_a_push, cost_after_button_a_push))
        {
            let mut new_state = ClawMachineState {
                coord: coord_after_button_a_push,
                cost: cost_after_button_a_push,
                button_a_pushes: current_state.button_a_pushes + 1,
                button_b_pushes: current_state.button_b_pushes,
                estimated_distance_to_end: 0,
            };

            new_state.set_estimated_distance_to_end(prize_coord);

            result.push(new_state);
        }
    }

    if current_state.button_b_pushes < 100 {
        let coord_after_button_b_push = Coordinate2D {
            x: current_state.coord.x + button_b_move_vector.x,
            y: current_state.coord.y + button_b_move_vector.y,
        };

        let cost_after_button_b_push = current_state.cost + 1;

        if coord_after_button_b_push.x <= prize_coord.x
            && coord_after_button_b_push.y <= prize_coord.y
            && !seen.contains(&(coord_after_button_b_push, cost_after_button_b_push))
        {
            let mut new_state = ClawMachineState {
                coord: coord_after_button_b_push,
                cost: cost_after_button_b_push,
                button_a_pushes: current_state.button_a_pushes,
                button_b_pushes: current_state.button_b_pushes + 1,
                estimated_distance_to_end: 0,
            };

            new_state.set_estimated_distance_to_end(prize_coord);

            result.push(new_state);
        }
    }

    result
}

fn read_button_move_vectors_and_prize_coords() -> Vec<(Coordinate2D, Coordinate2D, Coordinate2D)> {
    let mut result = vec![];

    let mut button_a_move_vector = None;
    let mut button_b_move_vector = None;
    let mut is_reading_claw_machine = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_claw_machine {
                button_a_move_vector = None;
                button_b_move_vector = None;
                is_reading_claw_machine = false;

                continue;
            }

            break;
        }

        if button_a_move_vector.is_none() || button_b_move_vector.is_none() {
            let x_start_index = trimmed_line.find('+').unwrap() + 1;
            let x_end_index = x_start_index + trimmed_line[x_start_index..].find(',').unwrap();
            let x = trimmed_line[x_start_index..x_end_index].parse().unwrap();

            let y_start_index = x_end_index + trimmed_line[x_end_index..].find('+').unwrap() + 1;
            let y = trimmed_line[y_start_index..].parse().unwrap();

            if button_a_move_vector.is_none() {
                button_a_move_vector = Some(Coordinate2D { x, y });
            } else {
                button_b_move_vector = Some(Coordinate2D { x, y });
            }
        } else {
            let x_start_index = trimmed_line.find('=').unwrap() + 1;
            let x_end_index = x_start_index + trimmed_line[x_start_index..].find(',').unwrap();
            let x = trimmed_line[x_start_index..x_end_index].parse().unwrap();

            let y_start_index = x_end_index + trimmed_line[x_end_index..].find('=').unwrap() + 1;
            let y = trimmed_line[y_start_index..].parse().unwrap();

            let prize_coord = Coordinate2D { x, y };

            result.push((
                button_a_move_vector.unwrap(),
                button_b_move_vector.unwrap(),
                prize_coord,
            ));
        }

        is_reading_claw_machine = true;
    }

    result
}

#[derive(Hash, PartialEq, Eq)]
struct ClawMachineState {
    coord: Coordinate2D,
    cost: u32,
    estimated_distance_to_end: u64,
    button_a_pushes: u32,
    button_b_pushes: u32,
}

impl ClawMachineState {
    fn get_score(&self) -> u64 {
        self.cost as u64 + self.estimated_distance_to_end
    }

    fn set_estimated_distance_to_end(&mut self, end: Coordinate2D) {
        self.estimated_distance_to_end =
            end.x.abs_diff(self.coord.x) + end.y.abs_diff(self.coord.y);
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate2D {
    x: i64,
    y: i64,
}

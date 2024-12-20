use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

fn main() {
    // solve_puzzle1(7, 12);
    // solve_puzzle1(71, 1024);
    // solve_puzzle2(7, 12);
    solve_puzzle2(71, 1024);
}

#[allow(dead_code)]
fn solve_puzzle2(memory_space_grid_size: usize, initial_falling_bytes_to_respect: usize) {
    let falling_byte_coords = read_falling_byte_coordinates();

    let mut memory_space_grid = build_memory_space_grid(
        memory_space_grid_size,
        &falling_byte_coords,
        initial_falling_bytes_to_respect,
    );

    let start = Coordinate2D { x: 0, y: 0 };

    let end = Coordinate2D {
        x: memory_space_grid_size - 1,
        y: memory_space_grid_size - 1,
    };

    for i in initial_falling_bytes_to_respect..falling_byte_coords.len() {
        let falling_byte_coord = falling_byte_coords[i];
        memory_space_grid[falling_byte_coord.y][falling_byte_coord.x] = '#';

        let min_steps = calculate_min_steps_to_end(&memory_space_grid, start, end);
        if min_steps.is_none() {
            println!("{},{}", falling_byte_coord.x, falling_byte_coord.y);
            return;
        }
    }
}

#[allow(dead_code)]
fn solve_puzzle1(memory_space_grid_size: usize, falling_bytes_to_respect: usize) {
    let falling_byte_coords = read_falling_byte_coordinates();

    let memory_space_grid = build_memory_space_grid(
        memory_space_grid_size,
        &falling_byte_coords,
        falling_bytes_to_respect,
    );

    let start = Coordinate2D { x: 0, y: 0 };

    let end = Coordinate2D {
        x: memory_space_grid_size - 1,
        y: memory_space_grid_size - 1,
    };

    match calculate_min_steps_to_end(&memory_space_grid, start, end) {
        Some(min_steps) => println!("{min_steps}"),
        None => println!("None"),
    }
}

// Implementation of A* search algorithm: https://en.wikipedia.org/wiki/A*_search_algorithm
fn calculate_min_steps_to_end(
    memory_space_grid: &[Vec<char>],
    start: Coordinate2D,
    end: Coordinate2D,
) -> Option<u32> {
    let mut states = PriorityQueue::with_capacity(2 * memory_space_grid.len());

    let mut initial_states = [
        PathState {
            coordinate: start,
            move_direction: MoveDirection::Down,
            steps: 0,
            estimated_distance_to_end: 0,
        },
        PathState {
            coordinate: start,
            move_direction: MoveDirection::Right,
            steps: 0,
            estimated_distance_to_end: 0,
        },
    ];

    initial_states.iter_mut().for_each(|s| {
        let state_score = s.get_score();
        states.push(s.clone(), Reverse(state_score));
    });

    let mut visited = HashSet::with_capacity(2 * memory_space_grid.len());

    while let Some((current_state, _)) = states.pop() {
        if current_state.coordinate == end {
            return Some(current_state.steps);
        }

        if !visited.insert((current_state.coordinate, current_state.move_direction)) {
            continue;
        }

        let next_states = calculate_next_states(&current_state, end, memory_space_grid, &visited);

        for next_state in next_states {
            let state_score = next_state.get_score();
            states.push(next_state, Reverse(state_score));
        }
    }

    None
}

fn calculate_next_states(
    current_state: &PathState,
    end: Coordinate2D,
    memory_space_grid: &[Vec<char>],
    visited: &HashSet<(Coordinate2D, MoveDirection)>,
) -> Vec<PathState> {
    let mut result = Vec::with_capacity(4);

    let coordinate = current_state.coordinate;
    let new_steps = current_state.steps + 1;

    if coordinate.y > 0 {
        let next_coordinate = Coordinate2D {
            y: coordinate.y - 1,
            ..coordinate
        };

        if memory_space_grid[next_coordinate.y][next_coordinate.x] == '.'
            && !visited.contains(&(next_coordinate, MoveDirection::Up))
        {
            result.push(PathState {
                coordinate: next_coordinate,
                move_direction: MoveDirection::Up,
                steps: new_steps,
                estimated_distance_to_end: 0,
            });
        }
    }

    if coordinate.y < memory_space_grid.len() - 1 {
        let next_coordinate = Coordinate2D {
            y: coordinate.y + 1,
            ..coordinate
        };

        if memory_space_grid[next_coordinate.y][next_coordinate.x] == '.'
            && !visited.contains(&(next_coordinate, MoveDirection::Down))
        {
            result.push(PathState {
                coordinate: next_coordinate,
                move_direction: MoveDirection::Down,
                steps: new_steps,
                estimated_distance_to_end: 0,
            });
        }
    }

    if coordinate.x > 0 {
        let next_coordinate = Coordinate2D {
            x: coordinate.x - 1,
            ..coordinate
        };

        if memory_space_grid[next_coordinate.y][next_coordinate.x] == '.'
            && !visited.contains(&(next_coordinate, MoveDirection::Left))
        {
            result.push(PathState {
                coordinate: next_coordinate,
                move_direction: MoveDirection::Left,
                steps: new_steps,
                estimated_distance_to_end: 0,
            });
        }
    }

    if coordinate.x < memory_space_grid.len() - 1 {
        let next_coordinate = Coordinate2D {
            x: coordinate.x + 1,
            ..coordinate
        };

        if memory_space_grid[next_coordinate.y][next_coordinate.x] == '.'
            && !visited.contains(&(next_coordinate, MoveDirection::Right))
        {
            result.push(PathState {
                coordinate: next_coordinate,
                move_direction: MoveDirection::Right,
                steps: new_steps,
                estimated_distance_to_end: 0,
            });
        }
    }

    result
        .iter_mut()
        .for_each(|s| s.set_estimated_distance_to_end(end));

    result
}

fn build_memory_space_grid(
    grid_size: usize,
    falling_byte_coords: &Vec<Coordinate2D>,
    falling_bytes_to_respect: usize,
) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; grid_size]; grid_size];

    for falling_byte_coord in falling_byte_coords.iter().take(falling_bytes_to_respect) {
        result[falling_byte_coord.y][falling_byte_coord.x] = '#';
    }

    result
}

fn read_falling_byte_coordinates() -> Vec<Coordinate2D> {
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

        let raw_coord = trimmed_line.split_once(',').unwrap();
        result.push(Coordinate2D {
            x: raw_coord.0.parse().unwrap(),
            y: raw_coord.1.parse().unwrap(),
        });
    }

    result
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct PathState {
    coordinate: Coordinate2D,
    move_direction: MoveDirection,
    steps: u32,
    estimated_distance_to_end: u32,
}

impl PathState {
    fn get_score(&self) -> u32 {
        self.steps + self.estimated_distance_to_end
    }

    fn set_estimated_distance_to_end(&mut self, end: Coordinate2D) {
        self.estimated_distance_to_end =
            end.y.abs_diff(self.coordinate.y) as u32 + end.x.abs_diff(self.coordinate.x) as u32;
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate2D {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

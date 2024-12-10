use std::collections::HashSet;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let topographic_map = read_topographic_map();

    let mut sum = 0;
    for row in 0..topographic_map.len() {
        for col in 0..topographic_map[row].len() {
            if topographic_map[row][col] == 0 {
                let mut trailhead_rating = 0;

                calculate_trailhead_rating(row, col, 0, &topographic_map, &mut trailhead_rating);

                sum += trailhead_rating;
            }
        }
    }

    println!("{sum}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let topographic_map = read_topographic_map();

    let mut sum = 0;
    for row in 0..topographic_map.len() {
        for col in 0..topographic_map[row].len() {
            if topographic_map[row][col] == 0 {
                let mut trailhead_score = 0;
                let mut visited_positions = HashSet::new();

                calculate_trailhead_score(
                    row,
                    col,
                    0,
                    &topographic_map,
                    &mut visited_positions,
                    &mut trailhead_score,
                );

                sum += trailhead_score;
            }
        }
    }

    println!("{sum}");
}

fn calculate_trailhead_score(
    current_row: usize,
    current_col: usize,
    current_height: u8,
    topographic_map: &[Vec<u8>],
    visited_positions: &mut HashSet<(usize, usize)>,
    trailhead_score: &mut usize,
) {
    visited_positions.insert((current_row, current_col));

    if current_height == 9 {
        *trailhead_score += 1;
        return;
    }

    let next_height = current_height + 1;

    if current_row < topographic_map.len() - 1 {
        let next_row = current_row + 1;
        if topographic_map[next_row][current_col] == next_height
            && !visited_positions.contains(&(next_row, current_col))
        {
            calculate_trailhead_score(
                next_row,
                current_col,
                next_height,
                topographic_map,
                visited_positions,
                trailhead_score,
            );
        }
    }

    if current_row > 0 {
        let next_row = current_row - 1;
        if topographic_map[next_row][current_col] == next_height
            && !visited_positions.contains(&(next_row, current_col))
        {
            calculate_trailhead_score(
                next_row,
                current_col,
                next_height,
                topographic_map,
                visited_positions,
                trailhead_score,
            );
        }
    }

    if current_col < topographic_map[0].len() - 1 {
        let next_col = current_col + 1;
        if topographic_map[current_row][next_col] == next_height
            && !visited_positions.contains(&(current_row, next_col))
        {
            calculate_trailhead_score(
                current_row,
                next_col,
                next_height,
                topographic_map,
                visited_positions,
                trailhead_score,
            );
        }
    }

    if current_col > 0 {
        let next_col = current_col - 1;
        if topographic_map[current_row][next_col] == next_height
            && !visited_positions.contains(&(current_row, next_col))
        {
            calculate_trailhead_score(
                current_row,
                next_col,
                next_height,
                topographic_map,
                visited_positions,
                trailhead_score,
            );
        }
    }
}

fn calculate_trailhead_rating(
    current_row: usize,
    current_col: usize,
    current_height: u8,
    topographic_map: &[Vec<u8>],
    trailhead_rating: &mut usize,
) {
    if current_height == 9 {
        *trailhead_rating += 1;
        return;
    }

    let next_height = current_height + 1;

    if current_row < topographic_map.len() - 1 {
        let next_row = current_row + 1;
        if topographic_map[next_row][current_col] == next_height {
            calculate_trailhead_rating(
                next_row,
                current_col,
                next_height,
                topographic_map,
                trailhead_rating,
            );
        }
    }

    if current_row > 0 {
        let next_row = current_row - 1;
        if topographic_map[next_row][current_col] == next_height {
            calculate_trailhead_rating(
                next_row,
                current_col,
                next_height,
                topographic_map,
                trailhead_rating,
            );
        }
    }

    if current_col < topographic_map[0].len() - 1 {
        let next_col = current_col + 1;
        if topographic_map[current_row][next_col] == next_height {
            calculate_trailhead_rating(
                current_row,
                next_col,
                next_height,
                topographic_map,
                trailhead_rating,
            );
        }
    }

    if current_col > 0 {
        let next_col = current_col - 1;
        if topographic_map[current_row][next_col] == next_height {
            calculate_trailhead_rating(
                current_row,
                next_col,
                next_height,
                topographic_map,
                trailhead_rating,
            );
        }
    }
}

fn read_topographic_map() -> Vec<Vec<u8>> {
    let mut map = Vec::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        map.push(
            trimmed_line
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect(),
        )
    }

    map
}

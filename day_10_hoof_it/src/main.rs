use std::collections::HashSet;

fn main() {
    solve_puzzle1();
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

                walk_hiking_trails(
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

fn walk_hiking_trails(
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
            walk_hiking_trails(
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
            walk_hiking_trails(
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
            walk_hiking_trails(
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
            walk_hiking_trails(
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

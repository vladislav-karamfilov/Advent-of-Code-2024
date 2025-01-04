use std::collections::HashMap;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/1#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let (left_location_ids, right_location_ids) = read_location_ids_lists();

    let mut similarity_scores_cache = HashMap::new();

    let mut total_similarity_score = 0;
    for left_location_id in left_location_ids.iter() {
        let similarity_score = similarity_scores_cache
            .entry(left_location_id)
            .or_insert_with(|| {
                left_location_id
                    * right_location_ids
                        .iter()
                        .filter(|rli| *rli == left_location_id)
                        .count() as i32
            });

        total_similarity_score += *similarity_score;
    }

    println!("{total_similarity_score}");
}

// https://adventofcode.com/2024/day/1
#[allow(dead_code)]
fn solve_puzzle1() {
    let (mut left_location_ids, mut right_location_ids) = read_location_ids_lists();

    left_location_ids.sort();
    right_location_ids.sort();

    let mut total_distance = 0;
    for (i, left_location_id) in left_location_ids.iter().enumerate() {
        let right_location_id = right_location_ids[i];
        let distance = (left_location_id - right_location_id).abs();
        total_distance += distance;
    }

    println!("{total_distance}");
}

fn read_location_ids_lists() -> (Vec<i32>, Vec<i32>) {
    let mut left_location_ids = vec![];
    let mut right_location_ids = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        let mut location_ids = trimmed_line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap());

        left_location_ids.push(location_ids.next().unwrap());
        right_location_ids.push(location_ids.next().unwrap());
    }

    (left_location_ids, right_location_ids)
}

use std::collections::HashMap;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/11#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let stones = read_stones();

    let mut stone_occurrences = HashMap::new();

    stones
        .iter()
        .for_each(|s| *stone_occurrences.entry(*s).or_default() += 1);

    for _ in 0..75 {
        stone_occurrences = transform_stones(stone_occurrences);
    }

    let count = stone_occurrences.values().sum::<u64>();
    println!("{count}");
}

// https://adventofcode.com/2024/day/11
#[allow(dead_code)]
fn solve_puzzle1() {
    let mut stones = read_stones();

    for _ in 0..25 {
        transform_stones_naive(&mut stones);
    }

    println!("{}", stones.len());
}

fn transform_stones(stone_occurrences: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result = HashMap::with_capacity(stone_occurrences.len());

    for (stone, occurrences) in stone_occurrences {
        if stone == 0 {
            let new_stone = 1;

            let new_stone_occurrences = result.entry(new_stone).or_default();
            *new_stone_occurrences += occurrences;
        } else if let Some((first_new_stone, second_new_stone)) = split_stone(stone) {
            let first_new_stone_occurrences = result.entry(first_new_stone).or_default();
            *first_new_stone_occurrences += occurrences;

            let second_new_stone_occurrences = result.entry(second_new_stone).or_default();
            *second_new_stone_occurrences += occurrences;
        } else {
            let new_stone = stone * 2024;

            let new_stone_occurrences = result.entry(new_stone).or_default();
            *new_stone_occurrences += occurrences;
        }
    }

    result
}

fn transform_stones_naive(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones[i];
        if stone == 0 {
            stones[i] = 1;
        } else if let Some((first_new_stone, second_new_stone)) = split_stone(stone) {
            stones[i] = first_new_stone;
            stones.insert(i + 1, second_new_stone);

            i += 1;
        } else {
            stones[i] = stone * 2024;
        }

        i += 1;
    }
}

fn split_stone(stone: u64) -> Option<(u64, u64)> {
    let digits = count_digits(stone);
    if digits % 2 != 0 {
        return None;
    }

    let half_digits = digits / 2;
    let divisor = 10u64.pow(half_digits);

    let first_new_stone = stone / divisor;
    let second_new_stone = stone % divisor;

    Some((first_new_stone, second_new_stone))
}

fn count_digits(number: u64) -> u32 {
    number.ilog10() + 1
}

fn read_stones() -> Vec<u64> {
    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

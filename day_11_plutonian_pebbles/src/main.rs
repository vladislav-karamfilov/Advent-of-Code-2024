fn main() {
    solve_puzzle1();
    // solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut stones = read_stones();

    for _ in 0..25 {
        transform_stones(&mut stones);
    }

    println!("{}", stones.len());
}

fn transform_stones(stones: &mut Vec<u64>) {
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

    let trimmed_line = line.trim();

    trimmed_line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

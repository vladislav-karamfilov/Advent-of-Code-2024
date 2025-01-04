use itertools::{iproduct, Itertools};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/22#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let initial_buyer_secret_numbers = read_initial_secret_numbers();

    let prices_and_price_changes_for_buyers =
        calculate_prices_and_price_changes_for_buyers(&initial_buyer_secret_numbers);

    let max_price_sum = iproduct!(-9_i8..10_i8, -9_i8..10_i8, -9_i8..10_i8, -9_i8..10_i8)
        .filter(|cv| {
            // Guess that the answer won't be in same number changes
            if cv.0 == cv.1 && cv.0 == cv.2 && cv.0 == cv.3 {
                return false;
            }

            // Guess that the answer won't be with all non-positive changes
            if cv.0 <= 0 && cv.1 <= 0 && cv.2 <= 0 && cv.3 <= 0 {
                return false;
            }

            true
        })
        .collect_vec()
        .par_chunks(10_000)
        .map(|changes_variations| {
            changes_variations
                .iter()
                .map(|changes_variation| {
                    prices_and_price_changes_for_buyers
                        .iter()
                        .map(|prices_and_price_changes_for_buyer| {
                            calculate_buyer_max_price_for_changes(
                                &prices_and_price_changes_for_buyer,
                                *changes_variation,
                            ) as u16
                        })
                        .sum::<u16>()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{max_price_sum}");
}

// https://adventofcode.com/2024/day/22
#[allow(dead_code)]
fn solve_puzzle1() {
    let initial_secret_numbers = read_initial_secret_numbers();

    let mut sum = 0;
    for initial_secret_number in initial_secret_numbers {
        let mut secret_number = initial_secret_number;
        for _ in 0..10 {
            secret_number = calculate_next_secret_number(secret_number);
        }

        sum += secret_number;
    }

    println!("{sum}");
}

fn calculate_buyer_max_price_for_changes(
    prices_and_price_changes_for_buyer: &[(u8, i8)],
    changes: (i8, i8, i8, i8),
) -> u8 {
    for i in 3..prices_and_price_changes_for_buyer.len() {
        if prices_and_price_changes_for_buyer[i - 3].1 == changes.0
            && prices_and_price_changes_for_buyer[i - 2].1 == changes.1
            && prices_and_price_changes_for_buyer[i - 1].1 == changes.2
            && prices_and_price_changes_for_buyer[i].1 == changes.3
        {
            return prices_and_price_changes_for_buyer[i].0;
        }
    }

    0
}

fn calculate_prices_and_price_changes_for_buyers(
    initial_secret_numbers_for_buyers: &[u64],
) -> Vec<Vec<(u8, i8)>> {
    let mut result = Vec::with_capacity(initial_secret_numbers_for_buyers.len());

    for initial_secret_number in initial_secret_numbers_for_buyers {
        let mut current_secret_number = *initial_secret_number;
        let mut current_buyer_price = (current_secret_number % 10) as u8;
        let mut prices_and_price_changes_for_buyer = Vec::with_capacity(2_000);

        for _ in 0..2_000 {
            let new_secret_number = calculate_next_secret_number(current_secret_number);
            let new_buyer_price = (new_secret_number % 10) as u8;
            let price_change = new_buyer_price as i8 - current_buyer_price as i8;

            prices_and_price_changes_for_buyer.push((new_buyer_price, price_change));

            current_secret_number = new_secret_number;
            current_buyer_price = new_buyer_price;
        }

        result.push(prices_and_price_changes_for_buyer);
    }

    result
}

fn calculate_next_secret_number(current_secret_number: u64) -> u64 {
    let mut next_secret_number = current_secret_number * 64;
    next_secret_number = mix_value_into_secret_number(next_secret_number, current_secret_number);
    next_secret_number = prune_secret_number(next_secret_number);

    let current_secret_number = next_secret_number;
    next_secret_number /= 32;
    next_secret_number = mix_value_into_secret_number(next_secret_number, current_secret_number);
    next_secret_number = prune_secret_number(next_secret_number);

    let current_secret_number = next_secret_number;
    next_secret_number *= 2048;
    next_secret_number = mix_value_into_secret_number(next_secret_number, current_secret_number);
    next_secret_number = prune_secret_number(next_secret_number);

    next_secret_number
}

fn mix_value_into_secret_number(secret_number: u64, value: u64) -> u64 {
    secret_number ^ value
}

fn prune_secret_number(secret_number: u64) -> u64 {
    secret_number % 16777216
}

fn read_initial_secret_numbers() -> Vec<u64> {
    let mut result = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        result.push(line.parse().unwrap());
    }

    result
}

fn main() {
    solve_puzzle1();
}

fn solve_puzzle1() {
    let initial_secret_numbers = read_initial_secret_numbers();

    let mut sum = 0;
    for initial_secret_number in initial_secret_numbers {
        let mut secret_number = initial_secret_number;
        for _ in 0..2_000 {
            secret_number = calculate_next_secret_number(secret_number);
        }

        sum += secret_number;
    }

    println!("{sum}");
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

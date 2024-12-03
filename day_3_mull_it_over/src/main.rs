use regex::Regex;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut result = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        let mut multiplication_enabled = true;
        let mut text_to_process = trimmed_line;
        loop {
            if multiplication_enabled {
                let disable_start_index = text_to_process.find("don't()");
                match disable_start_index {
                    Some(start_index) => {
                        result += calculate_sum_of_multiplications(&text_to_process[..start_index]);

                        text_to_process = &text_to_process[(start_index + 1)..];
                        multiplication_enabled = false;
                    }
                    None => {
                        result += calculate_sum_of_multiplications(text_to_process);

                        break;
                    }
                }
            } else {
                let enable_start_index = text_to_process.find("do()");
                match enable_start_index {
                    Some(start_index) => {
                        text_to_process = &text_to_process[(start_index + 1)..];
                        multiplication_enabled = true;
                    }
                    None => break,
                }
            }
        }
    }

    println!("{result}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut result = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        result += calculate_sum_of_multiplications(trimmed_line);
    }

    println!("{result}");
}

fn calculate_sum_of_multiplications(text_to_process: &str) -> u32 {
    let mut result = 0;
    if text_to_process.len() < "mul(X,Y)".len() {
        return result;
    }

    let multiplication_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for captures in multiplication_pattern.captures_iter(text_to_process) {
        let x = captures
            .get(1)
            .map(|m| m.as_str())
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let y = captures
            .get(2)
            .map(|m| m.as_str())
            .unwrap()
            .parse::<u32>()
            .unwrap();

        result += x * y;
    }

    result
}

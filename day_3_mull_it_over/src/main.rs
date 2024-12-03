use regex::Regex;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut result = 0;

    let program_memory = read_program_memory();

    let mut multiplication_enabled = true;
    let mut text_to_process = program_memory.as_str();
    loop {
        if multiplication_enabled {
            let disable_start_index = text_to_process.find("don't()");
            match disable_start_index {
                Some(start_index) => {
                    result += calculate_sum_of_multiplications(&text_to_process[..start_index]);

                    text_to_process = &text_to_process[start_index..];
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
                    text_to_process = &text_to_process[start_index..];
                    multiplication_enabled = true;
                }
                None => break,
            }
        }
    }

    println!("{result}");
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let program_memory = read_program_memory();

    let result = calculate_sum_of_multiplications(program_memory.as_str());

    println!("{result}");
}

fn read_program_memory() -> String {
    let mut result = String::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        if !result.is_empty() {
            result.push(' ');
        }

        result.push_str(trimmed_line);
    }

    result
}

fn calculate_sum_of_multiplications(text_to_process: &str) -> u32 {
    let mut result = 0;
    if text_to_process.len() < "mul(X,Y)".len() {
        return result;
    }

    let multiplication_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for curr_match in multiplication_pattern.captures_iter(text_to_process) {
        let x = curr_match
            .get(1)
            .map(|m| m.as_str())
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let y = curr_match
            .get(2)
            .map(|m| m.as_str())
            .unwrap()
            .parse::<u32>()
            .unwrap();

        result += x * y;
    }

    result
}

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/2#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let mut safe_reports_count = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        let levels = trimmed_line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe_report(&levels, -1) {
            safe_reports_count += 1;
        } else {
            for skip_level_index in 0..levels.len() {
                if is_safe_report(&levels, skip_level_index as i32) {
                    safe_reports_count += 1;
                    break;
                }
            }
        }
    }

    println!("{safe_reports_count}");
}

// https://adventofcode.com/2024/day/2
#[allow(dead_code)]
fn solve_puzzle1() {
    let mut safe_reports_count = 0;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        let levels = trimmed_line
            .split_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe_report(&levels, -1) {
            safe_reports_count += 1;
        }
    }

    println!("{safe_reports_count}");
}

fn is_safe_report(levels: &[i32], skip_level_index: i32) -> bool {
    let mut is_safe_report = true;
    let mut is_increasing = None;
    let mut previous_level = None;

    for (curr_level_index, curr_level) in levels.iter().enumerate() {
        if curr_level_index as i32 == skip_level_index {
            continue;
        }

        if let Some(prev_level) = previous_level {
            match is_increasing {
                Some(increasing) => {
                    if (increasing && (prev_level >= curr_level || curr_level - prev_level > 3))
                        || (!increasing
                            && (prev_level <= curr_level || prev_level - curr_level > 3))
                    {
                        is_safe_report = false;
                        break;
                    }
                }
                None => {
                    if prev_level < curr_level && curr_level - prev_level <= 3 {
                        is_increasing = Some(true);
                    } else if prev_level > curr_level && prev_level - curr_level <= 3 {
                        is_increasing = Some(false);
                    } else {
                        is_safe_report = false;
                        break;
                    }
                }
            }
        }

        previous_level = Some(curr_level);
    }

    is_safe_report
}

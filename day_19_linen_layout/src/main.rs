use std::collections::HashMap;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let (mut available_towel_patterns, designs) = read_available_towel_patterns_and_designs();

    // Have the longest towel patterns first so we can do the optimization heuristic below
    available_towel_patterns.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut cache = HashMap::with_capacity(5 * designs.len());

    let possible_designs = designs
        .iter()
        .filter(|d| is_design_possible(d, &available_towel_patterns, &mut cache))
        .count();

    println!("{possible_designs}");
}

fn is_design_possible(
    design: &str,
    available_towel_patterns: &[String],
    cache: &mut HashMap<String, bool>,
) -> bool {
    if let Some(is_possible) = cache.get(design) {
        return *is_possible;
    }

    for towel_pattern in available_towel_patterns {
        if towel_pattern == design {
            cache.insert(design.to_string(), true);

            return true;
        }

        if design.starts_with(towel_pattern) {
            let sub_design = &design[towel_pattern.len()..];
            if is_design_possible(sub_design, available_towel_patterns, cache) {
                return true;
            }

            // Optimization heuristic
            if towel_pattern.len() > 1
                && !available_towel_patterns
                    .iter()
                    .any(|p| p.len() < towel_pattern.len() && towel_pattern.starts_with(p))
            {
                cache.insert(sub_design.to_string(), false);

                // There are no shorter towel patterns that would match the current design start
                return false;
            }
        }
    }

    cache.insert(design.to_string(), false);

    false
}

fn read_available_towel_patterns_and_designs() -> (Vec<String>, Vec<String>) {
    let mut available_towel_patterns = vec![];
    let mut designs = vec![];

    let mut is_reading_available_towel_patterns = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_available_towel_patterns {
                is_reading_available_towel_patterns = false;

                continue;
            }

            break;
        }

        if is_reading_available_towel_patterns {
            available_towel_patterns.extend(trimmed_line.split(',').map(|x| x.trim().to_string()));
        } else {
            designs.push(trimmed_line.to_string());
        }
    }

    (available_towel_patterns, designs)
}

use std::{cmp::Ordering, collections::HashMap};

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/5#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let (page_ordering_rules, mut page_number_updates) =
        read_page_ordering_rules_and_page_number_updates();

    let mut result = 0;
    for update in page_number_updates.iter_mut() {
        if !is_correctly_ordered_page_number_update(update, &page_ordering_rules) {
            update.sort_by(|page_1, page_2| {
                if let Some(page_numbers_after) = page_ordering_rules.get(page_1) {
                    if page_numbers_after.contains(page_2) {
                        return Ordering::Greater;
                    }
                }

                if let Some(page_numbers_after) = page_ordering_rules.get(page_2) {
                    if page_numbers_after.contains(page_1) {
                        return Ordering::Less;
                    }
                }

                Ordering::Equal
            });

            result += update[update.len() / 2];
        }
    }

    println!("{result}");
}

// https://adventofcode.com/2024/day/5
#[allow(dead_code)]
fn solve_puzzle1() {
    let (page_ordering_rules, page_number_updates) =
        read_page_ordering_rules_and_page_number_updates();

    let mut result = 0;
    for update in page_number_updates.iter() {
        if is_correctly_ordered_page_number_update(update, &page_ordering_rules) {
            result += update[update.len() / 2];
        }
    }

    println!("{result}");
}

fn is_correctly_ordered_page_number_update(
    page_number_update: &[i32],
    page_ordering_rules: &HashMap<i32, Vec<i32>>,
) -> bool {
    for (index, page_number) in page_number_update.iter().enumerate() {
        if let Some(page_numbers_after) = page_ordering_rules.get(page_number) {
            let is_incorrectly_ordered_slice = page_number_update[..index]
                .iter()
                .any(|n| page_numbers_after.contains(n));

            if is_incorrectly_ordered_slice {
                return false;
            }
        }
    }

    true
}

fn read_page_ordering_rules_and_page_number_updates() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut page_ordering_rules = HashMap::new();
    let mut page_number_updates = vec![];

    let mut is_reading_page_ordering_rules = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_page_ordering_rules {
                is_reading_page_ordering_rules = false;
                continue;
            }

            break;
        }

        if is_reading_page_ordering_rules {
            let mut splitter = trimmed_line.split('|');
            let page_number = splitter.next().unwrap().parse::<i32>().unwrap();
            let page_number_after = splitter.next().unwrap().parse::<i32>().unwrap();

            let page_numbers_after = page_ordering_rules.entry(page_number).or_insert(vec![]);
            page_numbers_after.push(page_number_after);
        } else {
            page_number_updates.push(
                trimmed_line
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }

    (page_ordering_rules, page_number_updates)
}

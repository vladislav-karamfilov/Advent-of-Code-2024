use std::collections::HashMap;

fn main() {
    solve_puzzle1();
    // solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {}

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
        match page_ordering_rules.get(page_number) {
            Some(page_numbers_before) => {
                if page_number_update[0..index]
                    .iter()
                    .any(|n| page_numbers_before.contains(n))
                {
                    return false;
                }
            }
            None => {}
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
            let mut page_numbers = trimmed_line.split('|');
            let page_number = page_numbers.next().unwrap().parse::<i32>().unwrap();
            let page_number_before = page_numbers.next().unwrap().parse::<i32>().unwrap();

            let page_numbers_before = page_ordering_rules.entry(page_number).or_insert(vec![]);
            page_numbers_before.push(page_number_before);
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

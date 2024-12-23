use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let computer_network = read_computer_network();

    let count = count_target_subnetworks(&computer_network);

    println!("{count}");
}

fn count_target_subnetworks(computer_network: &HashMap<String, HashSet<String>>) -> usize {
    let mut target_subnetworks = HashSet::new();

    for (_, computers) in computer_network.iter().filter(|(c, _)| c.starts_with('t')) {
        let mut computer_combinations = computers
            .iter()
            .combinations(3)
            .filter(|cc| cc.iter().any(|c| c.starts_with('t')))
            .collect::<Vec<Vec<&String>>>();

        for computer_combination in computer_combinations.iter_mut() {
            if !is_each_computer_connected_to_others(computer_combination, computer_network) {
                continue;
            }

            computer_combination.sort();

            target_subnetworks.insert(computer_combination.into_iter().join(","));
        }
    }

    target_subnetworks.len()
}

fn is_each_computer_connected_to_others(
    computers: &[&String],
    computer_network: &HashMap<String, HashSet<String>>,
) -> bool {
    for (computer1, computer2) in computers.iter().tuple_combinations() {
        if !computer_network
            .get(*computer1)
            .unwrap()
            .contains(*computer2)
            || !computer_network
                .get(*computer2)
                .unwrap()
                .contains(*computer1)
        {
            return false;
        }
    }

    for computer in computers {
        if let Some(connected_computers) = computer_network.get(*computer) {
            for other_computer in computers {
                if computer != other_computer && !connected_computers.contains(*other_computer) {
                    return false;
                }
            }
        }
    }

    true
}

fn read_computer_network() -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let computers = line.split('-').collect::<Vec<&str>>();

        let first_computer_connected_computers =
            result.entry(computers[0].to_string()).or_default();

        // Add the computer itself to its connected computers set
        first_computer_connected_computers.insert(computers[0].to_string());
        first_computer_connected_computers.insert(computers[1].to_string());

        let second_computer_connected_computers =
            result.entry(computers[1].to_string()).or_default();

        // Add the computer itself to its connected computers set
        second_computer_connected_computers.insert(computers[1].to_string());
        second_computer_connected_computers.insert(computers[0].to_string());
    }

    result
}

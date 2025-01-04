use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

// https://adventofcode.com/2024/day/23#part2
#[allow(dead_code)]
fn solve_puzzle2() {
    let computer_network = read_computer_network();

    match get_lan_party_password(&computer_network) {
        Some(password) => println!("{password}"),
        None => println!("No password found"),
    }
}

// https://adventofcode.com/2024/day/23
#[allow(dead_code)]
fn solve_puzzle1() {
    let computer_network = read_computer_network();

    let count = count_target_subnetworks(&computer_network);

    println!("{count}");
}

fn get_lan_party_password(computer_network: &HashMap<String, HashSet<String>>) -> Option<String> {
    let max_computers_in_combination = computer_network.values().map(|cc| cc.len()).max().unwrap();

    for combination_count in (0..=max_computers_in_combination).rev() {
        for computers in computer_network.values() {
            let computer_combinations = computers.iter().combinations(combination_count);

            for computer_combination in computer_combinations {
                if is_each_computer_connected_to_others(
                    computer_combination.iter().map(|c| *c),
                    &computer_network,
                ) {
                    let mut computers_list = computer_combination.iter().collect::<Vec<_>>();
                    computers_list.sort();

                    let password = computers_list.into_iter().join(",");
                    return Some(password);
                }
            }
        }
    }

    None
}

fn count_target_subnetworks(computer_network: &HashMap<String, HashSet<String>>) -> usize {
    let mut target_subnetworks = HashSet::new();

    for (_, computers) in computer_network.iter().filter(|(c, _)| c.starts_with('t')) {
        let mut computer_combinations = computers
            .iter()
            .combinations(3)
            .filter(|cc| {
                cc.iter().any(|c| c.starts_with('t'))
                    && is_each_computer_connected_to_others(cc.iter().map(|c| *c), computer_network)
            })
            .collect::<Vec<_>>();

        for computer_combination in computer_combinations.iter_mut() {
            computer_combination.sort();

            target_subnetworks.insert(computer_combination.into_iter().join(","));
        }
    }

    target_subnetworks.len()
}

fn is_each_computer_connected_to_others<'a, I>(
    computers: I,
    computer_network: &HashMap<String, HashSet<String>>,
) -> bool
where
    I: Iterator<Item = &'a String>,
    I: Clone,
{
    for (computer1, computer2) in computers.tuple_combinations() {
        if !computer_network.get(computer1).unwrap().contains(computer2)
            || !computer_network.get(computer2).unwrap().contains(computer1)
        {
            return false;
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

        let computers = line.split('-').collect::<Vec<_>>();

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

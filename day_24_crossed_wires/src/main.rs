use std::collections::HashMap;

fn main() {
    solve_puzzle1();
}

// https://adventofcode.com/2024/day/24
#[allow(dead_code)]
fn solve_puzzle1() {
    let (mut wire_values, gate_connections) = read_initial_wire_values_and_gate_connections();

    simulate_all_gates(&gate_connections, &mut wire_values);

    let output_number = get_output_number(&wire_values);

    println!("{output_number}");
}

fn get_output_number(wire_values: &HashMap<String, u8>) -> u64 {
    let mut bits = vec![];
    let mut z_wire_counter = 0;
    while let Some(z_wire_value) = wire_values.get(&format!("z{:02}", z_wire_counter)) {
        bits.push(z_wire_value);
        z_wire_counter += 1;
    }

    let mut binary_str = String::with_capacity(bits.len());
    while let Some(bit) = bits.pop() {
        binary_str.push((bit + b'0') as char);
    }

    u64::from_str_radix(&binary_str, 2).unwrap()
}

fn simulate_all_gates(gate_connections: &[GateConnection], wire_values: &mut HashMap<String, u8>) {
    for gate_connection in gate_connections.iter() {
        let output_wire_value = simulate_gate(gate_connection, wire_values, &gate_connections);

        wire_values.insert(gate_connection.output_wire.clone(), output_wire_value);
    }
}

fn simulate_gate(
    gate_connection: &GateConnection,
    wire_values: &mut HashMap<String, u8>,
    gate_connections: &[GateConnection],
) -> u8 {
    let input_wire1_value = match wire_values.get(&gate_connection.input_wire1) {
        Some(input_wire1_value) => *input_wire1_value,
        None => {
            let gate_connection_to_simulate = gate_connections
                .iter()
                .find(|c| c.output_wire == gate_connection.input_wire1)
                .unwrap();

            let input_wire1_value =
                simulate_gate(gate_connection_to_simulate, wire_values, gate_connections);

            wire_values.insert(gate_connection.input_wire1.clone(), input_wire1_value);

            input_wire1_value
        }
    };

    let input_wire2_value = match wire_values.get(&gate_connection.input_wire2) {
        Some(input_wire2_value) => *input_wire2_value,
        None => {
            let gate_connection_to_simulate = gate_connections
                .iter()
                .find(|c| c.output_wire == gate_connection.input_wire2)
                .unwrap();

            let input_wire2_value =
                simulate_gate(gate_connection_to_simulate, wire_values, gate_connections);

            wire_values.insert(gate_connection.input_wire2.clone(), input_wire2_value);

            input_wire2_value
        }
    };

    match gate_connection.gate_type {
        GateType::And => input_wire1_value & input_wire2_value,
        GateType::Or => input_wire1_value | input_wire2_value,
        GateType::Xor => input_wire1_value ^ input_wire2_value,
    }
}

fn read_initial_wire_values_and_gate_connections() -> (HashMap<String, u8>, Vec<GateConnection>) {
    let mut initial_wire_values = HashMap::new();
    let mut gate_connections = vec![];
    let mut is_reading_initial_wire_values = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            if is_reading_initial_wire_values {
                is_reading_initial_wire_values = false;
                continue;
            }

            break;
        }

        if is_reading_initial_wire_values {
            let raw_wire_parts = line.split_once(": ").unwrap();

            initial_wire_values.insert(
                raw_wire_parts.0.to_string(),
                raw_wire_parts.1.parse::<u8>().unwrap(),
            );
        } else {
            let raw_gate_connection_parts = line.split_once(" -> ").unwrap();

            let mut raw_input_wires_and_gate_type_splitter =
                raw_gate_connection_parts.0.split_whitespace();

            let input_wire1 = raw_input_wires_and_gate_type_splitter.next().unwrap();
            let raw_gate_type = raw_input_wires_and_gate_type_splitter.next().unwrap();
            let input_wire2 = raw_input_wires_and_gate_type_splitter.next().unwrap();
            let gate_type = match raw_gate_type {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => unreachable!(),
            };

            gate_connections.push(GateConnection {
                input_wire1: input_wire1.to_string(),
                input_wire2: input_wire2.to_string(),
                output_wire: raw_gate_connection_parts.1.to_string(),
                gate_type,
            });
        }
    }

    (initial_wire_values, gate_connections)
}

struct GateConnection {
    input_wire1: String,
    input_wire2: String,
    output_wire: String,
    gate_type: GateType,
}

#[derive(Clone, Copy)]
enum GateType {
    And,
    Or,
    Xor,
}

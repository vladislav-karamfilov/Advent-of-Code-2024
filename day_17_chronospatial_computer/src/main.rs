fn main() {
    solve_puzzle1();
}

// https://adventofcode.com/2024/day/17
#[allow(dead_code)]
fn solve_puzzle1() {
    let mut program = read_program();

    let output = program.run();

    let result = output
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("{result}");
}

fn read_program() -> Program {
    let mut program = Program {
        instruction_opcodes: vec![],
        register_values: RegisterValues {
            a_value: 0,
            b_value: 0,
            c_value: 0,
        },
    };

    let mut is_reading_register_values = true;

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if is_reading_register_values {
                is_reading_register_values = false;

                continue;
            }

            break;
        }

        if is_reading_register_values {
            if let Some(register_a_start_index) = trimmed_line.find("A:") {
                program.register_values.a_value =
                    trimmed_line[register_a_start_index + 3..].parse().unwrap();
            } else if let Some(register_b_start_index) = trimmed_line.find("B:") {
                program.register_values.b_value =
                    trimmed_line[register_b_start_index + 3..].parse().unwrap();
            } else if let Some(register_c_start_index) = trimmed_line.find("C:") {
                program.register_values.c_value =
                    trimmed_line[register_c_start_index + 3..].parse().unwrap();
            }
        } else {
            let instruction_opcodes_start_index = trimmed_line.find(':').unwrap();
            program.instruction_opcodes = trimmed_line[instruction_opcodes_start_index + 2..]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
        }
    }

    program
}

struct Program {
    instruction_opcodes: Vec<u8>,
    register_values: RegisterValues,
}

struct RegisterValues {
    a_value: u32,
    b_value: u32,
    c_value: u32,
}

impl Program {
    fn run(&mut self) -> Vec<u32> {
        let mut output = vec![];

        let mut instruction_counter = 0;
        while instruction_counter < self.instruction_opcodes.len() {
            let instruction_opcode = self.instruction_opcodes[instruction_counter];
            let operand = self.instruction_opcodes[instruction_counter + 1];

            match instruction_opcode {
                0 => self.run_adv_instruction(operand),
                1 => self.run_bxl_instruction(operand),
                2 => self.run_bst_instruction(operand),
                3 => {
                    if let Some(new_instruction_counter) = self.run_jnz_instruction(operand) {
                        instruction_counter = new_instruction_counter;
                        continue;
                    }
                }
                4 => self.run_bxc_instruction(),
                5 => output.push(self.run_out_instruction(operand)),
                6 => self.run_bdv_instruction(operand),
                7 => self.run_cdv_instruction(operand),
                _ => unreachable!(),
            }

            instruction_counter += 2;
        }

        output
    }

    fn run_adv_instruction(&mut self, combo_operand: u8) {
        let numerator = self.register_values.a_value;
        let denominator = 2_u64.pow(self.determine_combo_operand_value(combo_operand)) as u32;

        self.register_values.a_value = numerator / denominator;
    }

    fn run_bxl_instruction(&mut self, literal_operand: u8) {
        self.register_values.b_value ^= literal_operand as u32;
    }

    fn run_bst_instruction(&mut self, combo_operand: u8) {
        self.register_values.b_value = self.determine_combo_operand_value(combo_operand) % 8;
    }

    fn run_jnz_instruction(&self, literal_operand: u8) -> Option<usize> {
        if self.register_values.a_value == 0 {
            return None;
        }

        Some(literal_operand.into())
    }

    fn run_bxc_instruction(&mut self) {
        self.register_values.b_value ^= self.register_values.c_value;
    }

    fn run_out_instruction(&self, combo_operand: u8) -> u32 {
        self.determine_combo_operand_value(combo_operand) % 8
    }

    fn run_bdv_instruction(&mut self, combo_operand: u8) {
        let numerator = self.register_values.a_value;
        let denominator = 2_u64.pow(self.determine_combo_operand_value(combo_operand)) as u32;

        self.register_values.b_value = numerator / denominator;
    }

    fn run_cdv_instruction(&mut self, combo_operand: u8) {
        let numerator = self.register_values.a_value;
        let denominator = 2_u64.pow(self.determine_combo_operand_value(combo_operand)) as u32;

        self.register_values.c_value = numerator / denominator;
    }

    fn determine_combo_operand_value(&self, combo_operand: u8) -> u32 {
        if combo_operand < 4 {
            combo_operand.into()
        } else if combo_operand == 4 {
            self.register_values.a_value
        } else if combo_operand == 5 {
            self.register_values.b_value
        } else if combo_operand == 6 {
            self.register_values.c_value
        } else {
            unreachable!()
        }
    }
}

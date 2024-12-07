fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let equations = read_equations();

    let result = equations
        .iter()
        .filter(|e| is_equation_possible(e))
        .map(|e| e.test_value)
        .sum::<u64>();

    println!("{result}");
}

fn is_equation_possible(equation: &Equation) -> bool {
    let operators_count = equation.operands.len() - 1;
    let operator_combinations_count = 1 << operators_count;
    let mut operators = vec![Operator::Add; operators_count];

    for i in 0..operator_combinations_count {
        for j in 0..operators_count {
            if (i & (1 << j)) != 0 {
                operators[j] = Operator::Multiply;
            }
        }

        if is_equation_true(equation, &operators) {
            return true;
        }

        operators.fill(Operator::Add);
    }

    false
}

fn is_equation_true(equation: &Equation, operators: &[Operator]) -> bool {
    let mut current_result = equation.operands[0];

    for (i, operator) in operators.iter().enumerate() {
        match operator {
            Operator::Add => current_result += equation.operands[i + 1],
            Operator::Multiply => current_result *= equation.operands[i + 1],
        }

        if current_result > equation.test_value {
            return false;
        }
    }

    current_result == equation.test_value
}

fn read_equations() -> Vec<Equation> {
    let mut result = vec![];

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        let mut splitter = trimmed_line.split(':');

        let test_value = splitter.next().unwrap().parse().unwrap();

        let operands = splitter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        result.push(Equation {
            test_value,
            operands,
        });
    }

    result
}

#[derive(Clone)]
enum Operator {
    Add,
    Multiply,
}

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

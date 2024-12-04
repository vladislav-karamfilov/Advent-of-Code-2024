fn main() {
    solve_puzzle1();
    // solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {}

#[allow(dead_code)]
fn solve_puzzle1() {
    let word_search = read_word_search();

    let max_col = word_search[0].len() - 1;

    let mut result = 0;
    for row in 0..word_search.len() {
        for col in 0..=max_col {
            if is_xmas_word(row, col, Direction::Right, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::Left, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::Up, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::Down, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::UpAndRight, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::UpAndLeft, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::DownAndRight, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::DownAndLeft, &word_search) {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn is_xmas_word(
    start_row: usize,
    start_col: usize,
    direction: Direction,
    word_search: &[Vec<char>],
) -> bool {
    let max_row = word_search[0].len() - 1;
    let max_col = word_search.len() - 1;

    match direction {
        Direction::Right => {
            start_col + 3 <= max_col
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row][start_col + 1] == 'M'
                && word_search[start_row][start_col + 2] == 'A'
                && word_search[start_row][start_col + 3] == 'S'
        }
        Direction::Left => {
            start_col >= 3
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row][start_col - 1] == 'M'
                && word_search[start_row][start_col - 2] == 'A'
                && word_search[start_row][start_col - 3] == 'S'
        }
        Direction::Up => {
            start_row >= 3
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row - 1][start_col] == 'M'
                && word_search[start_row - 2][start_col] == 'A'
                && word_search[start_row - 3][start_col] == 'S'
        }
        Direction::Down => {
            start_row + 3 <= max_row
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row + 1][start_col] == 'M'
                && word_search[start_row + 2][start_col] == 'A'
                && word_search[start_row + 3][start_col] == 'S'
        }
        Direction::UpAndRight => {
            start_row >= 3
                && start_col + 3 <= max_col
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row - 1][start_col + 1] == 'M'
                && word_search[start_row - 2][start_col + 2] == 'A'
                && word_search[start_row - 3][start_col + 3] == 'S'
        }
        Direction::UpAndLeft => {
            start_row >= 3
                && start_col >= 3
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row - 1][start_col - 1] == 'M'
                && word_search[start_row - 2][start_col - 2] == 'A'
                && word_search[start_row - 3][start_col - 3] == 'S'
        }
        Direction::DownAndRight => {
            start_row + 3 <= max_row
                && start_col + 3 <= max_col
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row + 1][start_col + 1] == 'M'
                && word_search[start_row + 2][start_col + 2] == 'A'
                && word_search[start_row + 3][start_col + 3] == 'S'
        }
        Direction::DownAndLeft => {
            start_row + 3 <= max_row
                && start_col >= 3
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row + 1][start_col - 1] == 'M'
                && word_search[start_row + 2][start_col - 2] == 'A'
                && word_search[start_row + 3][start_col - 3] == 'S'
        }
    }
}

fn read_word_search() -> Vec<Vec<char>> {
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

        result.push(trimmed_line.chars().collect());
    }

    result
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpAndRight,
    UpAndLeft,
    DownAndRight,
    DownAndLeft,
}

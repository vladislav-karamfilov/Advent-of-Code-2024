fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let word_search = read_word_search();

    let max_col = word_search[0].len() - 1;

    let mut result = 0;
    for row in 0..word_search.len() {
        for col in 0..=max_col {
            if is_x_mas_word(row, col, &word_search) {
                result += 1;
            }
        }
    }

    println!("{result}");
}

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

            if is_xmas_word(row, col, Direction::UpRight, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::UpLeft, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::DownRight, &word_search) {
                result += 1;
            }

            if is_xmas_word(row, col, Direction::DownLeft, &word_search) {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn is_x_mas_word(start_row: usize, start_col: usize, word_search: &[Vec<char>]) -> bool {
    if word_search[start_row][start_col] != 'A' {
        return false; // A must be in the center of the X-MAS
    }

    let max_row = word_search[0].len() - 1;
    let max_col = word_search.len() - 1;

    if start_row == 0 || start_row == max_row || start_col == 0 || start_col == max_col {
        return false; // Some of the neighbor cells would be outside of the boundaries
    }

    let up_left = word_search[start_row - 1][start_col - 1];
    let up_right = word_search[start_row - 1][start_col + 1];
    let down_left = word_search[start_row + 1][start_col - 1];
    let down_right = word_search[start_row + 1][start_col + 1];

    ((up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M'))
        && ((up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M'))
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
        Direction::UpRight => {
            start_row >= 3
                && start_col + 3 <= max_col
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row - 1][start_col + 1] == 'M'
                && word_search[start_row - 2][start_col + 2] == 'A'
                && word_search[start_row - 3][start_col + 3] == 'S'
        }
        Direction::UpLeft => {
            start_row >= 3
                && start_col >= 3
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row - 1][start_col - 1] == 'M'
                && word_search[start_row - 2][start_col - 2] == 'A'
                && word_search[start_row - 3][start_col - 3] == 'S'
        }
        Direction::DownRight => {
            start_row + 3 <= max_row
                && start_col + 3 <= max_col
                && word_search[start_row][start_col] == 'X'
                && word_search[start_row + 1][start_col + 1] == 'M'
                && word_search[start_row + 2][start_col + 2] == 'A'
                && word_search[start_row + 3][start_col + 3] == 'S'
        }
        Direction::DownLeft => {
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
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

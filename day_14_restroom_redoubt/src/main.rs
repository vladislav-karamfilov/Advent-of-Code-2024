fn main() {
    // solve_puzzle1(10, 6);
    solve_puzzle1(100, 102);
}

#[allow(dead_code)]
fn solve_puzzle1(max_x: i32, max_y: i32) {
    let mut robots = read_robots();

    (0..100).for_each(|_| {
        robots
            .iter_mut()
            .for_each(|r| r.simulate_move(max_x, max_y))
    });

    let safety_factor = calculate_safety_factor(&robots, max_x, max_y);

    println!("{safety_factor}");
}

fn calculate_safety_factor(robots: &Vec<Robot>, max_x: i32, max_y: i32) -> usize {
    let mid_x = max_x / 2;
    let mid_y = max_y / 2;

    let first_quadrant_robots = robots
        .iter()
        .filter(|r| r.position.x < mid_x && r.position.y < mid_y)
        .count();

    let second_quadrant_robots = robots
        .iter()
        .filter(|r| r.position.x > mid_x && r.position.y < mid_y)
        .count();

    let third_quadrant_robots = robots
        .iter()
        .filter(|r| r.position.x < mid_x && r.position.y > mid_y)
        .count();

    let fourth_quadrant_robots = robots
        .iter()
        .filter(|r| r.position.x > mid_x && r.position.y > mid_y)
        .count();

    let safety_factor = first_quadrant_robots
        * second_quadrant_robots
        * third_quadrant_robots
        * fourth_quadrant_robots;

    safety_factor
}

fn read_robots() -> Vec<Robot> {
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

        let mut position_and_velocity_splitter = trimmed_line.split('=');
        position_and_velocity_splitter.next();

        let raw_position = position_and_velocity_splitter
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.split_whitespace().next().unwrap().parse().unwrap())
            .collect::<Vec<i32>>();

        let raw_velocity = position_and_velocity_splitter
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        let position = Coordinate2D {
            x: raw_position[0],
            y: raw_position[1],
        };

        let velocity = Coordinate2D {
            x: raw_velocity[0],
            y: raw_velocity[1],
        };

        result.push(Robot { position, velocity });
    }

    result
}

#[allow(dead_code)]
fn print_robots(robots: &Vec<Robot>, max_x: i32, max_y: i32) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            let count = robots
                .iter()
                .filter(|r| r.position.x == x && r.position.y == y)
                .count();

            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
        }

        println!();
    }
}

struct Robot {
    position: Coordinate2D,
    velocity: Coordinate2D,
}

impl Robot {
    fn simulate_move(&mut self, max_x: i32, max_y: i32) {
        let mut next_x = self.position.x + self.velocity.x;
        if next_x < 0 {
            next_x += max_x + 1;
        } else if next_x > max_x {
            next_x -= max_x + 1;
        }

        let mut next_y = self.position.y + self.velocity.y;
        if next_y < 0 {
            next_y += max_y + 1;
        } else if next_y > max_y {
            next_y -= max_y + 1;
        }

        self.position = Coordinate2D {
            x: next_x,
            y: next_y,
        };
    }
}

struct Coordinate2D {
    x: i32,
    y: i32,
}

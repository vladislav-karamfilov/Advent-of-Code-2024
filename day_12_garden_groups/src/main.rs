use std::collections::HashSet;

fn main() {
    solve_puzzle1();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let garden_plots_map = read_garden_plots_map();

    let garden_plot_regions = find_garden_plot_regions(&garden_plots_map);

    let total_price_of_fencing = garden_plot_regions
        .iter()
        .map(|r| {
            let area = r.positions.len();
            let perimeter = calculate_garden_plots_region_perimeter(r, &garden_plots_map);

            area * perimeter
        })
        .sum::<usize>();

    println!("{total_price_of_fencing}");
}

fn calculate_garden_plots_region_perimeter(
    garden_plots_region: &GardenPlotRegion,
    garden_plots_map: &[Vec<char>],
) -> usize {
    let mut result = 0;

    for position in garden_plots_region.positions.iter() {
        if position.row == 0
            || garden_plots_map[position.row - 1][position.col] != garden_plots_region.plant_type
        {
            result += 1;
        }

        if position.row == garden_plots_map.len() - 1
            || garden_plots_map[position.row + 1][position.col] != garden_plots_region.plant_type
        {
            result += 1;
        }

        if position.col == 0
            || garden_plots_map[position.row][position.col - 1] != garden_plots_region.plant_type
        {
            result += 1;
        }

        if position.col == garden_plots_map[0].len() - 1
            || garden_plots_map[position.row][position.col + 1] != garden_plots_region.plant_type
        {
            result += 1;
        }
    }

    result
}

fn find_garden_plot_regions(garden_plots_map: &[Vec<char>]) -> Vec<GardenPlotRegion> {
    let mut result = vec![];

    let mut visited_positions =
        HashSet::with_capacity(garden_plots_map.len() * garden_plots_map[0].len());

    for (row, plant_types) in garden_plots_map.iter().enumerate() {
        for (col, plant_type) in plant_types.iter().enumerate() {
            let current_position = Position { row, col };
            if visited_positions.contains(&current_position) {
                continue;
            }

            let mut new_garden_plots_region = GardenPlotRegion {
                plant_type: *plant_type,
                positions: HashSet::new(),
            };

            find_garden_plots_in_region(
                current_position,
                garden_plots_map,
                &mut new_garden_plots_region,
            );

            new_garden_plots_region.positions.iter().for_each(|p| {
                visited_positions.insert(*p);
            });

            result.push(new_garden_plots_region);
        }
    }

    result
}

fn find_garden_plots_in_region(
    current_position: Position,
    garden_plots_map: &[Vec<char>],
    garden_plots_region: &mut GardenPlotRegion,
) {
    garden_plots_region.positions.insert(current_position);

    if current_position.row < garden_plots_map.len() - 1 {
        let next_position = Position {
            row: current_position.row + 1,
            col: current_position.col,
        };

        if garden_plots_map[next_position.row][next_position.col] == garden_plots_region.plant_type
            && !garden_plots_region.positions.contains(&next_position)
        {
            find_garden_plots_in_region(next_position, garden_plots_map, garden_plots_region);
        }
    }

    if current_position.row > 0 {
        let next_position = Position {
            row: current_position.row - 1,
            col: current_position.col,
        };

        if garden_plots_map[next_position.row][next_position.col] == garden_plots_region.plant_type
            && !garden_plots_region.positions.contains(&next_position)
        {
            find_garden_plots_in_region(next_position, garden_plots_map, garden_plots_region);
        }
    }

    if current_position.col < garden_plots_map[0].len() - 1 {
        let next_position = Position {
            row: current_position.row,
            col: current_position.col + 1,
        };

        if garden_plots_map[next_position.row][next_position.col] == garden_plots_region.plant_type
            && !garden_plots_region.positions.contains(&next_position)
        {
            find_garden_plots_in_region(next_position, garden_plots_map, garden_plots_region);
        }
    }

    if current_position.col > 0 {
        let next_position = Position {
            row: current_position.row,
            col: current_position.col - 1,
        };

        if garden_plots_map[next_position.row][next_position.col] == garden_plots_region.plant_type
            && !garden_plots_region.positions.contains(&next_position)
        {
            find_garden_plots_in_region(next_position, garden_plots_map, garden_plots_region);
        }
    }
}

fn read_garden_plots_map() -> Vec<Vec<char>> {
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

struct GardenPlotRegion {
    plant_type: char,
    positions: HashSet<Position>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

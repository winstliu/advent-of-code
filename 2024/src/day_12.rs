use std::collections::{HashSet, VecDeque};

pub fn part_1(contents: &str) -> Result<u64, String> {
    let grid = parse_grid(contents);
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

    let mut total_price = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, plot) in line.iter().enumerate() {
            if visited_positions.contains(&(row, col)) {
                continue;
            }

            let region_positions = get_region_positions(&grid, *plot, row, col);
            let area: u64 = region_positions
                .len()
                .try_into()
                .map_err(|err: std::num::TryFromIntError| err.to_string())?;
            let perimeter = region_positions
                .iter()
                .map(|(row, col)| get_plot_boundary_length(&grid, *plot, *row, *col))
                .sum::<u64>();

            total_price += area * perimeter;

            visited_positions.extend(region_positions.iter());
        }
    }

    Ok(total_price)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    Ok(0)
}

fn parse_grid(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_region_positions(
    grid: &[Vec<char>],
    plot: char,
    row: usize,
    col: usize,
) -> HashSet<(usize, usize)> {
    let mut visited_positions = HashSet::new();
    let mut positions_to_visit = VecDeque::from([(row, col)]);

    while let Some((row, col)) = positions_to_visit.pop_front() {
        if visited_positions.contains(&(row, col)) || grid[row][col] != plot {
            continue;
        }

        visited_positions.insert((row, col));
        if row != 0 {
            positions_to_visit.push_back((row - 1, col));
        }

        if row != grid.len() - 1 {
            positions_to_visit.push_back((row + 1, col));
        }

        if col != 0 {
            positions_to_visit.push_back((row, col - 1));
        }

        if col != grid[row].len() - 1 {
            positions_to_visit.push_back((row, col + 1));
        }
    }

    visited_positions
}

fn get_plot_boundary_length(grid: &[Vec<char>], plot: char, row: usize, col: usize) -> u64 {
    let mut length = 0;

    if row == 0 || grid[row - 1][col] != plot {
        length += 1;
    }

    if row == grid.len() - 1 || grid[row + 1][col] != plot {
        length += 1;
    }

    if col == 0 || grid[row][col - 1] != plot {
        length += 1;
    }

    if col == grid[0].len() - 1 || grid[row][col + 1] != plot {
        length += 1;
    }

    length
}

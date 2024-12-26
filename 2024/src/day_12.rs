use std::collections::{HashMap, HashSet, VecDeque};

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
                .map(|(row, col)| get_plot_sides(&grid, *plot, *row, *col))
                .sum::<u64>();

            total_price += area * perimeter;

            visited_positions.extend(region_positions.iter());
        }
    }

    Ok(total_price)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
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
            let sides = get_region_sides(&grid, &region_positions);

            total_price += area * sides;

            visited_positions.extend(region_positions.iter());
        }
    }

    Ok(total_price)
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

fn get_region_sides(grid: &[Vec<char>], region_positions: &HashSet<(usize, usize)>) -> u64 {
    let mut sides_to_explore = region_positions
        .iter()
        .filter_map(|(row, col)| {
            let sides = get_plot_sides(grid, grid[*row][*col], *row, *col);
            if sides == 0 {
                None
            } else {
                Some((
                    (*row, *col),
                    (
                        sides, // number of unexplored sides
                        HashSet::new(), // already explored sides
                    ),
                ))
            }
        })
        .collect::<HashMap<_, _>>();

    let mut total_sides = 0;
    while let Some(((starting_row, starting_col), (_, explored_sides))) =
        sides_to_explore.iter().next()
    {
        let starting_row = *starting_row;
        let starting_col = *starting_col;

        let mut row = starting_row;
        let mut col = starting_col;
        let mut direction;
        let mut side;

        if !explored_sides.contains(&Side::Top)
            && (starting_row == 0 || !region_positions.contains(&(starting_row - 1, starting_col)))
        {
            direction = Direction::Right;
            side = Side::Top;
        } else if !explored_sides.contains(&Side::Right)
            && (starting_col == grid[starting_row].len() - 1
                || !region_positions.contains(&(starting_row, starting_col + 1)))
        {
            direction = Direction::Down;
            side = Side::Right;
        } else if !explored_sides.contains(&Side::Bottom)
            && (starting_row == grid.len() - 1
                || !region_positions.contains(&(starting_row + 1, starting_col)))
        {
            direction = Direction::Left;
            side = Side::Bottom;
        } else if !explored_sides.contains(&Side::Left)
            && (starting_col == 0 || !region_positions.contains(&(starting_row, starting_col - 1)))
        {
            direction = Direction::Up;
            side = Side::Left;
        } else {
            panic!(
                "Could not determine a starting orientation for ({}, {})",
                starting_row, starting_col
            );
        }

        let starting_direction = direction;
        let starting_side = side;
        loop {
            let Some((sides, explored_sides)) = sides_to_explore.get_mut(&(row, col)) else {
                panic!("({}, {}) is not in the current garden", row, col);
            };
            *sides -= 1;
            explored_sides.insert(side);

            match direction {
                Direction::Up => {
                    if side == Side::Right {
                        if row > 0
                            && col < grid[row - 1].len() - 1
                            && sides_to_explore.contains_key(&(row - 1, col + 1))
                        {
                            row -= 1;
                            col += 1;
                            direction = Direction::Right;
                            side = Side::Bottom;

                            total_sides += 1;
                        } else if row > 0 && sides_to_explore.contains_key(&(row - 1, col)) {
                            row -= 1;
                        } else {
                            direction = Direction::Left;
                            side = Side::Top;

                            total_sides += 1;
                        }
                    } else if side == Side::Left {
                        if row > 0 && col > 0 && sides_to_explore.contains_key(&(row - 1, col - 1))
                        {
                            row -= 1;
                            col -= 1;
                            direction = Direction::Left;
                            side = Side::Bottom;

                            total_sides += 1;
                        } else if row > 0 && sides_to_explore.contains_key(&(row - 1, col)) {
                            row -= 1;
                        } else {
                            direction = Direction::Right;
                            side = Side::Top;

                            total_sides += 1;
                        }
                    }
                }
                Direction::Right => {
                    if side == Side::Top {
                        if row > 0
                            && col < grid[row - 1].len() - 1
                            && sides_to_explore.contains_key(&(row - 1, col + 1))
                        {
                            row -= 1;
                            col += 1;
                            direction = Direction::Up;
                            side = Side::Left;

                            total_sides += 1;
                        } else if col < grid[row].len() - 1
                            && sides_to_explore.contains_key(&(row, col + 1))
                        {
                            col += 1;
                        } else {
                            direction = Direction::Down;
                            side = Side::Right;

                            total_sides += 1;
                        }
                    } else if side == Side::Bottom {
                        if row < grid.len() - 1
                            && col < grid[row + 1].len() - 1
                            && sides_to_explore.contains_key(&(row + 1, col + 1))
                        {
                            row += 1;
                            col += 1;
                            direction = Direction::Down;
                            side = Side::Left;

                            total_sides += 1;
                        } else if col < grid[row].len() - 1
                            && sides_to_explore.contains_key(&(row, col + 1))
                        {
                            col += 1;
                        } else {
                            direction = Direction::Up;
                            side = Side::Right;

                            total_sides += 1;
                        }
                    }
                }
                Direction::Down => {
                    if side == Side::Right {
                        if row < grid.len() - 1
                            && col < grid[row + 1].len() - 1
                            && sides_to_explore.contains_key(&(row + 1, col + 1))
                        {
                            row += 1;
                            col += 1;
                            direction = Direction::Right;
                            side = Side::Top;

                            total_sides += 1;
                        } else if row < grid.len() - 1
                            && sides_to_explore.contains_key(&(row + 1, col))
                        {
                            row += 1;
                        } else {
                            direction = Direction::Left;
                            side = Side::Bottom;

                            total_sides += 1;
                        }
                    } else if side == Side::Left {
                        if row < grid.len() - 1
                            && col > 0
                            && sides_to_explore.contains_key(&(row + 1, col - 1))
                        {
                            row += 1;
                            col -= 1;
                            direction = Direction::Left;
                            side = Side::Top;

                            total_sides += 1;
                        } else if row < grid.len() - 1
                            && sides_to_explore.contains_key(&(row + 1, col))
                        {
                            row += 1;
                        } else {
                            direction = Direction::Right;
                            side = Side::Bottom;

                            total_sides += 1;
                        }
                    }
                }
                Direction::Left => {
                    if side == Side::Top {
                        if row > 0 && col > 0 && sides_to_explore.contains_key(&(row - 1, col - 1))
                        {
                            row -= 1;
                            col -= 1;
                            direction = Direction::Up;
                            side = Side::Right;

                            total_sides += 1;
                        } else if col > 0 && sides_to_explore.contains_key(&(row, col - 1)) {
                            col -= 1;
                        } else {
                            direction = Direction::Down;
                            side = Side::Left;

                            total_sides += 1;
                        }
                    } else if side == Side::Bottom {
                        if row < grid.len() - 1
                            && col > 0
                            && sides_to_explore.contains_key(&(row + 1, col - 1))
                        {
                            row += 1;
                            col -= 1;
                            direction = Direction::Down;
                            side = Side::Right;

                            total_sides += 1;
                        } else if col > 0 && sides_to_explore.contains_key(&(row, col - 1)) {
                            col -= 1;
                        } else {
                            direction = Direction::Up;
                            side = Side::Left;

                            total_sides += 1;
                        }
                    }
                }
            }

            if row == starting_row
                && col == starting_col
                && direction == starting_direction
                && side == starting_side
            {
                break;
            }
        }

        sides_to_explore.retain(|_, (sides, _)| *sides > 0);
    }

    total_sides
}

fn get_plot_sides(grid: &[Vec<char>], plot: char, row: usize, col: usize) -> u64 {
    let mut sides = 0;

    if row == 0 || grid[row - 1][col] != plot {
        sides += 1;
    }

    if row == grid.len() - 1 || grid[row + 1][col] != plot {
        sides += 1;
    }

    if col == 0 || grid[row][col - 1] != plot {
        sides += 1;
    }

    if col == grid[row].len() - 1 || grid[row][col + 1] != plot {
        sides += 1;
    }

    sides
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

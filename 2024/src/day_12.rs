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
                .map(|(row, col)| get_plot_edges(&grid, *plot, *row, *col))
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
    let mut positions_with_edges = region_positions
        .iter()
        .filter_map(|(row, col)| {
            let edges = get_plot_edges(grid, grid[*row][*col], *row, *col);
            if edges == 0 {
                None
            } else {
                Some((
                    (*row, *col),
                    get_plot_edges(grid, grid[*row][*col], *row, *col),
                ))
            }
        })
        .collect::<HashMap<_, _>>();

    let mut num_total_edges: u64 = 0;
    while let Some(((starting_row, starting_col), edges)) = positions_with_edges.iter().next() {
        let starting_row = *starting_row;
        let starting_col = *starting_col;
        let edges = *edges;

        let mut row = starting_row;
        let mut col = starting_col;
        let mut direction;
        let mut edge_position;

        // v col
        // 0AAAAA <-- row
        // ACABBA
        // AAABBA
        // ABBAAA
        // ABBAAA
        // AAAAAn
        // ...D..

        // AAAA
        // ABBA
        // AAAA

        // BAAAB
        // BAAAB
        // BAAAB
        // BBBBB

        // AAAAA
        // AAAAA
        // ABABA
        // ABBBA
        // AAAAA

        // 21112
        // 11011
        // 2 3 2
        // 2   2
        // 22222

        // 00000
        // 01010
        // 1 3 1
        // X   1
        // 01110

        // WINSTON DO NOT REMOVE EMPTY EDGES FROM MAP
        // At X, Direction::Up, EdgePosition::Right
        // if region.contains(row - 1, col + 1) && edges(row - 1, col + 1) > 0
        //      turn right (Direction::Right, EdgePosition::Bottom)
        // else if region.contains(row - 1, col) && edges(row - 1, col) > 0
        //      go up (same direction/edgePostion)
        // else
        //      turn left (Direction::Left, EdgePosition::Top)

        if starting_row == 0
            || grid[starting_row - 1][starting_col] != grid[starting_row][starting_col]
        {
            direction = Direction::Right;
            edge_position = EdgePosition::Top;
        } else if starting_row == grid.len() - 1
            || grid[starting_row + 1][starting_col] != grid[starting_row][starting_col]
        {
            direction = Direction::Left;
            edge_position = EdgePosition::Bottom;
        } else if starting_col == 0
            || grid[starting_row][starting_col - 1] != grid[starting_row][starting_col]
        {
            direction = Direction::Up;
            edge_position = EdgePosition::Left;
        } else if starting_col == grid[starting_row].len() - 1
            || grid[starting_row][starting_col + 1] != grid[starting_row][starting_col]
        {
            direction = Direction::Down;
            edge_position = EdgePosition::Right;
        } else {
            panic!("No edge found at ({}, {})", starting_row, starting_col);
        }

        num_total_edges += 1;

        let starting_direction = direction;
        loop {
            positions_with_edges.insert((row, col), edges - 1);

            if direction == Direction::Up {
                if edge_position == EdgePosition::Right {
                    if row > 0
                        && col < grid[row - 1].len() - 1
                        && positions_with_edges.contains_key(&(row - 1, col + 1))
                    {
                        row -= 1;
                        col += 1;
                        direction = Direction::Right;
                        edge_position = EdgePosition::Bottom;

                        num_total_edges += 1;
                    } else if row > 0 && positions_with_edges.contains_key(&(row - 1, col)) {
                        row -= 1;
                    } else {
                        direction = Direction::Left;
                        edge_position = EdgePosition::Top;

                        num_total_edges += 1;
                    }
                } else if edge_position == EdgePosition::Left {
                    if row > 0 && col > 0 && positions_with_edges.contains_key(&(row - 1, col - 1))
                    {
                        row -= 1;
                        col -= 1;
                        direction = Direction::Left;
                        edge_position = EdgePosition::Bottom;

                        num_total_edges += 1;
                    } else if row > 0 && positions_with_edges.contains_key(&(row - 1, col)) {
                        row -= 1;
                    } else {
                        direction = Direction::Right;
                        edge_position = EdgePosition::Top;

                        num_total_edges += 1;
                    }
                }
            } else if direction == Direction::Right {
                if edge_position == EdgePosition::Top {
                    if row > 0
                        && col < grid[row - 1].len() - 1
                        && positions_with_edges.contains_key(&(row - 1, col + 1))
                    {
                        row -= 1;
                        col += 1;
                        direction = Direction::Up;
                        edge_position = EdgePosition::Left;

                        num_total_edges += 1;
                    } else if col < grid[row].len() - 1
                        && positions_with_edges.contains_key(&(row, col + 1))
                    {
                        col += 1;
                    } else {
                        direction = Direction::Down;
                        edge_position = EdgePosition::Right;

                        num_total_edges += 1;
                    }
                } else if edge_position == EdgePosition::Bottom {
                    if row < grid.len() - 1
                        && col < grid[row + 1].len() - 1
                        && positions_with_edges.contains_key(&(row + 1, col + 1))
                    {
                        row += 1;
                        col += 1;
                        direction = Direction::Down;
                        edge_position = EdgePosition::Left;

                        num_total_edges += 1;
                    } else if col < grid[row].len() - 1
                        && positions_with_edges.contains_key(&(row, col + 1))
                    {
                        col += 1;
                    } else {
                        direction = Direction::Up;
                        edge_position = EdgePosition::Right;

                        num_total_edges += 1;
                    }
                }
            } else if direction == Direction::Down {
                if edge_position == EdgePosition::Right {
                    if row < grid.len() - 1
                        && col < grid[row + 1].len() - 1
                        && positions_with_edges.contains_key(&(row + 1, col + 1))
                    {
                        row += 1;
                        col += 1;
                        direction = Direction::Right;
                        edge_position = EdgePosition::Top;

                        num_total_edges += 1;
                    } else if row < grid.len() - 1
                        && positions_with_edges.contains_key(&(row + 1, col))
                    {
                        row += 1;
                    } else {
                        direction = Direction::Left;
                        edge_position = EdgePosition::Bottom;

                        num_total_edges += 1;
                    }
                } else if edge_position == EdgePosition::Left {
                    if row < grid.len() - 1
                        && col > 0
                        && positions_with_edges.contains_key(&(row + 1, col - 1))
                    {
                        row += 1;
                        col -= 1;
                        direction = Direction::Left;
                        edge_position = EdgePosition::Top;

                        num_total_edges += 1;
                    } else if row < grid.len() - 1
                        && positions_with_edges.contains_key(&(row + 1, col))
                    {
                        row += 1;
                    } else {
                        direction = Direction::Right;
                        edge_position = EdgePosition::Bottom;

                        num_total_edges += 1;
                    }
                }
            } else if direction == Direction::Left {
                if edge_position == EdgePosition::Top {
                    if row > 0 && col > 0 && positions_with_edges.contains_key(&(row - 1, col - 1))
                    {
                        row -= 1;
                        col -= 1;
                        direction = Direction::Up;
                        edge_position = EdgePosition::Right;

                        num_total_edges += 1;
                    } else if col > 0 && positions_with_edges.contains_key(&(row, col - 1)) {
                        col -= 1;
                    } else {
                        direction = Direction::Down;
                        edge_position = EdgePosition::Left;

                        num_total_edges += 1;
                    }
                } else if edge_position == EdgePosition::Bottom {
                    if row < grid.len() - 1
                        && col > 0
                        && positions_with_edges.contains_key(&(row + 1, col - 1))
                    {
                        row += 1;
                        col -= 1;
                        direction = Direction::Down;
                        edge_position = EdgePosition::Right;

                        num_total_edges += 1;
                    } else if col > 0 && positions_with_edges.contains_key(&(row, col - 1)) {
                        col -= 1;
                    } else {
                        direction = Direction::Up;
                        edge_position = EdgePosition::Left;

                        num_total_edges += 1;
                    }
                }
            }

            if row == starting_row && col == starting_col {
                if direction == starting_direction {
                    num_total_edges -= 1;
                }
                break;
            }
        }

        positions_with_edges.retain(|_, edges| *edges > 0);
    }

    num_total_edges
}

fn get_plot_edges(grid: &[Vec<char>], plot: char, row: usize, col: usize) -> u64 {
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

    if col == grid[row].len() - 1 || grid[row][col + 1] != plot {
        length += 1;
    }

    length
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum EdgePosition {
    Top,
    Bottom,
    Left,
    Right,
}

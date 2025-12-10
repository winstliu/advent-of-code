use std::collections::HashSet;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let grid = parse_grid(contents)?;
    Ok(grid.iter().enumerate().fold(0, |acc, (row_index, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (col_index, &cell)| {
            if cell {
                let mut adjacent_rolls = 0;

                // Top
                if row_index > 0 && grid[row_index - 1][col_index] {
                    adjacent_rolls += 1;
                }

                // Top-right
                if row_index > 0 && col_index + 1 < row.len() && grid[row_index - 1][col_index + 1]
                {
                    adjacent_rolls += 1;
                }

                // Right
                if col_index + 1 < row.len() && grid[row_index][col_index + 1] {
                    adjacent_rolls += 1;
                }

                // Bottom-right
                if row_index + 1 < grid.len()
                    && col_index + 1 < row.len()
                    && grid[row_index + 1][col_index + 1]
                {
                    adjacent_rolls += 1;
                }

                // Bottom
                if row_index + 1 < grid.len() && grid[row_index + 1][col_index] {
                    adjacent_rolls += 1;
                }

                // Bottom-left
                if row_index + 1 < grid.len() && col_index > 0 && grid[row_index + 1][col_index - 1]
                {
                    adjacent_rolls += 1;
                }

                // Left
                if col_index > 0 && grid[row_index][col_index - 1] {
                    adjacent_rolls += 1;
                }

                // Top-left
                if row_index > 0 && col_index > 0 && grid[row_index - 1][col_index - 1] {
                    adjacent_rolls += 1;
                }

                if adjacent_rolls < 4 {
                    return acc + 1;
                }
            }

            acc
        })
    }))
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let (row_len, col_len) = get_grid_dimensions(contents)?;
    let mut paper_roll_locations = get_paper_roll_locations(contents);
    let mut total_removed = 0;
    loop {
        let removed_paper_rolls =
            &paper_roll_locations
                .clone()
                .into_iter()
                .fold(0, |acc, (row, col)| {
                    let mut adjacent_rolls = 0;

                    // Top
                    if row > 0 && paper_roll_locations.contains(&(row - 1, col)) {
                        adjacent_rolls += 1;
                    }

                    // Top-right
                    if row > 0
                        && col + 1 < col_len
                        && paper_roll_locations.contains(&(row - 1, col + 1))
                    {
                        adjacent_rolls += 1;
                    }

                    // Right
                    if col + 1 < col_len && paper_roll_locations.contains(&(row, col + 1)) {
                        adjacent_rolls += 1;
                    }

                    // Bottom-right
                    if row + 1 < row_len
                        && col + 1 < col_len
                        && paper_roll_locations.contains(&(row + 1, col + 1))
                    {
                        adjacent_rolls += 1;
                    }

                    // Bottom
                    if row + 1 < row_len && paper_roll_locations.contains(&(row + 1, col)) {
                        adjacent_rolls += 1;
                    }

                    // Bottom-left
                    if row + 1 < row_len
                        && col > 0
                        && paper_roll_locations.contains(&(row + 1, col - 1))
                    {
                        adjacent_rolls += 1;
                    }

                    // Left
                    if col > 0 && paper_roll_locations.contains(&(row, col - 1)) {
                        adjacent_rolls += 1;
                    }

                    // Top-left
                    if row > 0 && col > 0 && paper_roll_locations.contains(&(row - 1, col - 1)) {
                        adjacent_rolls += 1;
                    }

                    if adjacent_rolls < 4 {
                        paper_roll_locations.remove(&(row, col));
                        return acc + 1;
                    }

                    acc
                });

        total_removed += removed_paper_rolls;

        if *removed_paper_rolls == 0 {
            break;
        }
    }

    Ok(total_removed)
}

fn parse_grid(contents: &str) -> Result<Vec<Vec<bool>>, String> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Ok(false),
                    '@' => Ok(true),
                    _ => Err(format!("Invalid character in grid: {}", c)),
                })
                .collect()
        })
        .collect()
}

fn get_grid_dimensions(contents: &str) -> Result<(usize, usize), String> {
    let row_len = contents.lines().count();
    let col_len = contents.lines().next().ok_or("Empty grid")?.chars().count();
    Ok((row_len, col_len))
}

fn get_paper_roll_locations(contents: &str) -> HashSet<(usize, usize)> {
    contents
        .lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col_index, cell)| match cell {
                    '.' => None,
                    '@' => Some((row_index, col_index)),
                    _ => None,
                })
        })
        .collect()
}

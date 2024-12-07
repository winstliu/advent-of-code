use std::collections::HashSet;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let (grid, starting_position) = try_parse_grid(contents)?;

    let mut visited_positions = HashSet::from([starting_position]);

    let (mut row, mut col) = starting_position;
    let mut direction = Direction::North;
    loop {
        match direction {
            Direction::North => {
                if row == 0 {
                    break;
                }

                if grid[row - 1][col] == Grid::Obstruction {
                    direction = Direction::East;
                    continue;
                }

                row -= 1;

                visited_positions.insert((row, col));
            }
            Direction::East => {
                if col == grid[row].len() - 1 {
                    break;
                }

                if grid[row][col + 1] == Grid::Obstruction {
                    direction = Direction::South;
                    continue;
                }

                col += 1;

                visited_positions.insert((row, col));
            }
            Direction::South => {
                if row == grid.len() - 1 {
                    break;
                }

                if grid[row + 1][col] == Grid::Obstruction {
                    direction = Direction::West;
                    continue;
                }

                row += 1;

                visited_positions.insert((row, col));
            }
            Direction::West => {
                if col == 0 {
                    break;
                }

                if grid[row][col - 1] == Grid::Obstruction {
                    direction = Direction::North;
                    continue;
                }

                col -= 1;

                visited_positions.insert((row, col));
            }
        }
    }

    Ok(visited_positions.len() as u64)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    Ok(0)
}

fn try_parse_grid(contents: &str) -> Result<(Vec<Vec<Grid>>, Position), String> {
    let mut starting_position = None;
    let grid = contents
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Ok(Grid::Path),
                    '^' => {
                        starting_position = Some((row, col));
                        Ok(Grid::Path)
                    }
                    '#' => Ok(Grid::Obstruction),
                    _ => Err(format!("Unexpected character {} in grid", c).to_string()),
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let Some(starting_position) = starting_position else {
        return Err("No starting position found in grid".to_string());
    };

    Ok((grid, starting_position))
}

#[derive(PartialEq, Eq)]
enum Grid {
    Path,
    Obstruction,
}

enum Direction {
    North,
    East,
    South,
    West,
}

type Position = (usize, usize);

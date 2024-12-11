use std::collections::HashSet;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let grid = try_parse_grid(contents)?;
    Ok(grid.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (x, _)| {
            acc + find_trailends(&grid, x, y, 0).len() as u64
        })
    }))
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let grid = try_parse_grid(contents)?;
    Ok(grid.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, _)| acc + find_trails(&grid, x, y, 0))
    }))
}

fn try_parse_grid(contents: &str) -> Result<Vec<Vec<u32>>, String> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| {
                    height
                        .to_digit(10)
                        .ok_or(format!("Could not parse {} as u32", height))
                })
                .collect()
        })
        .collect()
}

fn find_trailends(grid: &[Vec<u32>], x: usize, y: usize, height: u32) -> HashSet<(usize, usize)> {
    let mut trailends = HashSet::new();
    if grid[y][x] != height {
        return trailends;
    }

    if grid[y][x] == 9 {
        trailends.insert((x, y));
        return trailends;
    }

    if x > 0 {
        trailends.extend(find_trailends(grid, x - 1, y, height + 1));
    }

    if x < grid[y].len() - 1 {
        trailends.extend(find_trailends(grid, x + 1, y, height + 1));
    }

    if y > 0 {
        trailends.extend(find_trailends(grid, x, y - 1, height + 1));
    }

    if y < grid.len() - 1 {
        trailends.extend(find_trailends(grid, x, y + 1, height + 1));
    }

    trailends
}

fn find_trails(grid: &[Vec<u32>], x: usize, y: usize, height: u32) -> u64 {
    if grid[y][x] != height {
        return 0;
    }

    if grid[y][x] == 9 {
        return 1;
    }

    let mut trails = 0;
    if x > 0 {
        trails += find_trails(grid, x - 1, y, height + 1);
    }

    if x < grid[y].len() - 1 {
        trails += find_trails(grid, x + 1, y, height + 1);
    }

    if y > 0 {
        trails += find_trails(grid, x, y - 1, height + 1);
    }

    if y < grid.len() - 1 {
        trails += find_trails(grid, x, y + 1, height + 1);
    }

    trails
}

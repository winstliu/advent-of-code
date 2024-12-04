pub fn part_1(contents: &str) -> Result<u64, String> {
    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(grid.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, _)| acc + find_xmas(&grid, x, y))
    }))
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(grid.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter(|(x, _)| find_x_mas(&grid, *x, y))
            .count() as u64
    }))
}

fn find_xmas(grid: &[Vec<char>], x: usize, y: usize) -> u64 {
    let mut matches = 0;

    if grid[x][y] != 'X' {
        return 0;
    }

    // Top-left
    if x > 2
        && y > 2
        && grid[x - 1][y - 1] == 'M'
        && grid[x - 2][y - 2] == 'A'
        && grid[x - 3][y - 3] == 'S'
    {
        matches += 1;
    }

    // Top
    if y > 2 && grid[x][y - 1] == 'M' && grid[x][y - 2] == 'A' && grid[x][y - 3] == 'S' {
        matches += 1;
    }

    // Top-right
    if x < grid[x].len() - 3
        && y > 2
        && grid[x + 1][y - 1] == 'M'
        && grid[x + 2][y - 2] == 'A'
        && grid[x + 3][y - 3] == 'S'
    {
        matches += 1;
    }

    // Left
    if x > 2 && grid[x - 1][y] == 'M' && grid[x - 2][y] == 'A' && grid[x - 3][y] == 'S' {
        matches += 1;
    }

    // Right
    if x < grid[x].len() - 3
        && grid[x + 1][y] == 'M'
        && grid[x + 2][y] == 'A'
        && grid[x + 3][y] == 'S'
    {
        matches += 1;
    }

    // Bottom-left
    if x > 2
        && y < grid.len() - 3
        && grid[x - 1][y + 1] == 'M'
        && grid[x - 2][y + 2] == 'A'
        && grid[x - 3][y + 3] == 'S'
    {
        matches += 1;
    }

    // Bottom
    if y < grid.len() - 3 && grid[x][y + 1] == 'M' && grid[x][y + 2] == 'A' && grid[x][y + 3] == 'S'
    {
        matches += 1;
    }

    // Bottom-right
    if x < grid[x].len() - 3
        && y < grid.len() - 3
        && grid[x + 1][y + 1] == 'M'
        && grid[x + 2][y + 2] == 'A'
        && grid[x + 3][y + 3] == 'S'
    {
        matches += 1;
    }

    matches
}

fn find_x_mas(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if grid[x][y] != 'A' {
        return false;
    }

    // M M
    //  A
    // S S
    if x > 0
        && y > 0
        && x < grid[x].len() - 1
        && y < grid.len() - 1
        && grid[x - 1][y - 1] == 'M'
        && grid[x + 1][y + 1] == 'S'
        && grid[x + 1][y - 1] == 'M'
        && grid[x - 1][y + 1] == 'S'
    {
        return true;
    }

    // M S
    //  A
    // M S
    if x > 0
        && y > 0
        && x < grid[x].len() - 1
        && y < grid.len() - 1
        && grid[x - 1][y - 1] == 'M'
        && grid[x + 1][y + 1] == 'S'
        && grid[x + 1][y - 1] == 'S'
        && grid[x - 1][y + 1] == 'M'
    {
        return true;
    }

    // S S
    //  A
    // M M
    if x > 0
        && y > 0
        && x < grid[x].len() - 1
        && y < grid.len() - 1
        && grid[x - 1][y - 1] == 'S'
        && grid[x + 1][y + 1] == 'M'
        && grid[x + 1][y - 1] == 'S'
        && grid[x - 1][y + 1] == 'M'
    {
        return true;
    }

    // S M
    //  A
    // S M
    if x > 0
        && y > 0
        && x < grid[x].len() - 1
        && y < grid.len() - 1
        && grid[x - 1][y - 1] == 'S'
        && grid[x + 1][y + 1] == 'M'
        && grid[x + 1][y - 1] == 'M'
        && grid[x - 1][y + 1] == 'S'
    {
        return true;
    }

    false
}

pub fn part_1(contents: &str) -> Result<u64, String> {
    let (mut left, mut right) = try_parse_input(contents)?;

    left.sort();
    right.sort();
    Ok(left.iter().zip(right).fold(0, |acc, (l, r)| {
        acc + r.abs_diff(*l)
    }))
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let (left, right) = try_parse_input(contents)?;

    Ok(left.iter().fold(0, |acc, l| {
        let matches = right.iter().filter(|r| l == *r).count() as u64;
        acc + l * matches
    }))
}

fn try_parse_input(contents: &str) -> Result<(Vec<u64>, Vec<u64>), String> {
    contents.lines().map(|line| {
        let mut iter = line.split_ascii_whitespace().take(2);
        Ok((
            iter
                .next().ok_or("No first element")?
                .parse::<u64>().map_err(|err| err.to_string())?,
            iter
                .next().ok_or("No second element")?
                .parse::<u64>().map_err(|err| err.to_string())?
        ))
    }).collect()
}

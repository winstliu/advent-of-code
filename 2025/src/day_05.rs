pub fn part_1(contents: &str) -> Result<u64, String> {
    let (fresh_ranges, ids) = contents.split_once("\n\n").ok_or("Invalid input")?;

    let fresh_ranges = parse_fresh_ranges(fresh_ranges)?;
    let ids = parse_ingredient_ids(ids)?;

    let mut fresh_ids = 0;
    for id in ids {
        for range in &fresh_ranges {
            if id >= range.0 && id <= range.1 {
                fresh_ids += 1;
                break;
            }
        }
    }

    Ok(fresh_ids)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let (fresh_ranges, _) = contents.split_once("\n\n").ok_or("Invalid input")?;

    let fresh_ranges = parse_fresh_ranges(fresh_ranges)?;
    let fresh_ranges = deduplicate_ranges(&fresh_ranges);

    Ok(fresh_ranges
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .sum::<u64>())
}

fn parse_fresh_ranges(contents: &str) -> Result<Vec<(u64, u64)>, String> {
    contents
        .lines()
        .map(|line| {
            line.split_once('-')
                .ok_or("Invalid range".to_string())
                .and_then(|(start, end)| {
                    let start = start
                        .parse::<u64>()
                        .map_err(|_| "Invalid number".to_string())?;
                    let end = end
                        .parse::<u64>()
                        .map_err(|_| "Invalid number".to_string())?;
                    Ok((start, end))
                })
        })
        .collect::<Result<_, _>>()
}

fn parse_ingredient_ids(contents: &str) -> Result<Vec<u64>, String> {
    contents
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|_| "Invalid ingredient ID".to_string())
        })
        .collect::<Result<_, _>>()
}

fn deduplicate_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut ranges = ranges.to_vec();
    ranges.sort_unstable();

    ranges
        .iter()
        .skip(1)
        .fold(vec![ranges[0]], |mut acc, range| {
            let last_index = acc.len() - 1;
            let (start1, end1) = acc[last_index];
            let (start2, end2) = *range;

            // Because of sorting, start1 is always <= start2

            if end1 >= start2 {
                // Range 1 ends after range 2 starts, so they overlap
                // |--------|
                //    |--------|
                // or
                // |--------|
                //   |----|
                acc[last_index] = (start1, end1.max(end2));
            } else {
                // No overlap
                // |--------|
                //            |--------|
                acc.push((start2, end2));
            }

            acc
        })
}

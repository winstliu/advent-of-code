pub fn part_1(contents: &str) -> Result<u64, String> {
    let [rules, updates] = contents.split("\r\n\r\n").collect::<Vec<_>>()[..] else {
        return Err("Invalid input".to_string());
    };

    let rules = rules
        .lines()
        .map(try_parse_rule)
        .collect::<Result<Vec<_>, _>>()?;

    updates.lines().try_fold(0, |acc, line| {
        let pages = line
            .split(',')
            .map(|page| page.parse::<u64>().map_err(|err| err.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        if are_pages_sorted(&pages, &rules) {
            return Ok(acc + pages[(pages.len() - 1) / 2]);
        }

        Ok(acc)
    })
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let [rules, updates] = contents.split("\r\n\r\n").collect::<Vec<_>>()[..] else {
        return Err("Invalid input".to_string());
    };

    let rules = rules
        .lines()
        .map(try_parse_rule)
        .collect::<Result<Vec<_>, _>>()?;

    let invalid_updates = updates.lines().filter_map(|line| {
        let Ok(pages) = line
            .split(',')
            .map(|page| page.parse::<u64>().map_err(|err| err.to_string()))
            .collect::<Result<Vec<_>, _>>() else { return None; };
        if !are_pages_sorted(&pages, &rules) {
            return Some(pages);
        }

        None
    });

    Ok(invalid_updates.fold(0, |acc, pages| {
        acc + 1
    }))
}

fn try_parse_rule(rule: &str) -> Result<(u64, u64), String> {
    let mut iter = rule.split('|');
    Ok((
        iter.next()
            .ok_or("No first page")?
            .parse::<u64>()
            .map_err(|err| err.to_string())?,
        iter.next()
            .ok_or("No second page")?
            .parse::<u64>()
            .map_err(|err| err.to_string())?,
    ))
}

fn are_pages_sorted(pages: &[u64], rules: &[(u64, u64)]) -> bool {
    pages.iter().enumerate().all(|(i, page)| {
        rules
            .iter()
            .filter(|(a, b)| page == a || page == b)
            .all(|(a, b)| {
                if page == a {
                    if let Some(j) = pages.iter().position(|p| p == b) {
                        return i < j;
                    }

                    return true;
                }

                if page == b {
                    if let Some(j) = pages.iter().position(|p| p == a) {
                        return i > j;
                    }

                    return true;
                }

                true
            })
    })
}

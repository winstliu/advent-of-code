use regex::Regex;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").map_err(|err| err.to_string())?;
    let result = regex.captures_iter(contents).try_fold(0, |acc, c| {
        let a = c.get(1).ok_or("No first capture group")?.as_str().parse::<u64>().map_err(|err| err.to_string())?;
        let b = c.get(2).ok_or("No second capture group")?.as_str().parse::<u64>().map_err(|err| err.to_string())?;

        Ok(acc + a * b)
    });

    result
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut process_muls = true;

    let regex = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))").map_err(|err| err.to_string())?;
    let result = regex.captures_iter(contents).try_fold(0, |acc, c| {
        if c.name("do").is_some() {
            process_muls = true;
            return Ok(acc);
        } else if c.name("dont").is_some() {
            process_muls = false;
            return Ok(acc);
        }

        if !process_muls {
            return Ok(acc);
        }

        let a = c.get(1).ok_or("No first capture group")?.as_str().parse::<u64>().map_err(|err| err.to_string())?;
        let b = c.get(2).ok_or("No second capture group")?.as_str().parse::<u64>().map_err(|err| err.to_string())?;

        Ok(acc + a * b)
    });

    result
}

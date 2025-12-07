pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut result: i64 = 50;
    let mut num_dial_ends_at_zero = 0;
    for line in contents.lines() {
        let (direction, amount) = line.split_at(1);

        let amount = amount.parse::<i64>().map_err(|e| e.to_string())?;
        result += match direction {
            "L" => -amount,
            "R" => amount,
            _ => return Err("Invalid direction".to_string()),
        };

        result = result.rem_euclid(100);

        if result == 0 {
            num_dial_ends_at_zero += 1;
        }
    }

    Ok(num_dial_ends_at_zero)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut result: i64 = 50;
    let mut num_dial_passes_zero: u64 = 0;
    for line in contents.lines() {
        let (direction, amount) = line.split_at(1);

        let amount = amount.parse::<u64>().map_err(|e| e.to_string())?;
        num_dial_passes_zero += amount / 100;
        let amount = amount % 100;

        let new_result = match direction {
            "L" => result.strict_sub_unsigned(amount),
            "R" => result.strict_add_unsigned(amount),
            _ => return Err("Invalid direction".to_string()),
        };

        if result != 0 && !(0..=100).contains(&new_result) {
            num_dial_passes_zero += 1;
        }

        result = new_result.rem_euclid(100);

        if result == 0 {
            num_dial_passes_zero += 1;
        }
    }

    Ok(num_dial_passes_zero)
}

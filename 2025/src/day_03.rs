pub fn part_1(contents: &str) -> Result<u64, String> {
    contents.lines().try_fold(0, |sum, line| {
        let chars = line.chars();
        let (pos, largest_first_position) = line
            .chars()
            .take(chars.count() - 1)
            .enumerate()
            .try_fold((0, 0), |(i, max), c| {
                let num = c.1.to_digit(10).ok_or("Not a digit")?.into();
                if num > max {
                    Ok::<_, String>((c.0, num))
                } else {
                    Ok((i, max))
                }
            })?;

        let largest_last_position = line.chars().skip(pos + 1).try_fold(0, |max, c| {
            let num = c.to_digit(10).ok_or("Not a digit")?.into();
            if num > max {
                Ok::<_, String>(num)
            } else {
                Ok(max)
            }
        })?;

        Ok(sum + largest_first_position * 10 + largest_last_position)
    })
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    Ok(contents
        .lines()
        .map(|line| {
            let length = line.chars().count();
            let (_, s) = (0..=11)
                .rev()
                .try_fold((0, 0), |(current_pos, sum), num_left| {
                    let (pos, largest_valid_digit) = line
                        .chars()
                        .take(length - num_left)
                        .enumerate()
                        .skip(current_pos)
                        .try_fold((0, 0), |(i, max), c| {
                            let num = c.1.to_digit(10).ok_or("Not a digit")?.into();
                            if num > max {
                                Ok::<_, String>((c.0, num))
                            } else {
                                Ok((i, max))
                            }
                        })?;

                    Ok::<_, String>((
                        pos + 1,
                        sum + largest_valid_digit * 10_u64.pow(num_left as u32),
                    ))
                })?;

            Ok::<_, String>(s)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
}

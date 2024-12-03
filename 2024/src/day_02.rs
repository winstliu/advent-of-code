pub fn part_1(contents: &str) -> Result<u64, String> {
    contents.lines().try_fold(0, |acc, line| {
        let readings = try_parse_readings(line)?;

        // First figure out if we are descending or ascending
        let is_descending = readings[1] < readings[0];

        // Then check if the readings don't change too fast
        let result = readings.windows(2).all(|pair| {
            is_safe(pair[0], pair[1], is_descending)
        });
        Ok(acc + result as u64)
    })
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    contents.lines().try_fold(0, |acc, line| {
        let readings = try_parse_readings(line)?;

        let diffs = readings.windows(2).map(|pair| {
            pair[1] as i64 - pair[0] as i64
        });

        let num_negative = diffs.clone().filter(|&val| val < 0).count();
        let num_positive = diffs.clone().filter(|&val| val > 0).count();

        let is_descending = num_negative > num_positive;

        let mut used_problem_dampener = false;

        let mut index_to_remove = -1;

        let very_first_pair_is_safe = is_safe(readings[0], readings[1], is_descending);
        if !very_first_pair_is_safe {
            used_problem_dampener = true;
        }

        let very_second_pair_is_safe = is_safe(readings[1], readings[2], is_descending);
        if !very_first_pair_is_safe && !very_second_pair_is_safe {
            if is_safe(readings[0], readings[2], is_descending) {
                index_to_remove = 0;
                used_problem_dampener = true;
            } else {
                println!("FAIL A: {}", line);
                return Ok(acc);
            }
        }

        let result = readings[1..].windows(3).all(|triple| {
            let first_pair_is_safe = if index_to_remove >= 0 {
                true
            } else {
                is_safe(triple[0], triple[1], is_descending)
            };

            let second_pair_is_safe = if index_to_remove == 1 {
                is_safe(triple[0], triple[2], is_descending)
            } else {
                is_safe(triple[1], triple[2], is_descending)
            };
            index_to_remove -= 1;

            if first_pair_is_safe && second_pair_is_safe {
                return true;
            }

            if used_problem_dampener {
                println!("FAIL B: {}", line);
                return false;
            }

            if !first_pair_is_safe {
                if is_safe(triple[0], triple[2], is_descending) {
                    index_to_remove = 0;
                    used_problem_dampener = true;
                } else {
                    println!("FAIL C: {}", line);
                    return false;
                }
            }

            if !second_pair_is_safe {
                index_to_remove = 1;
                used_problem_dampener = true;
            }

            true
        });

        Ok(acc + result as u64)
    })
}

fn try_parse_readings(line: &str) -> Result<Vec<u64>, String> {
    line.split_ascii_whitespace()
        .map(|val| val.parse::<u64>().map_err(|err| err.to_string()))
        .collect()
}

fn is_safe(a: u64, b: u64, is_descending: bool) -> bool {
    if (is_descending && b >= a) || (!is_descending && b <= a) {
        return false;
    }
    b.abs_diff(a) <= 3
}

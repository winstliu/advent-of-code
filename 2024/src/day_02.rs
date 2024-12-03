pub fn part_1(contents: &str) -> Result<u64, String> {
    contents.lines().filter(|line| {
        let Ok(readings) = try_parse_readings(line) else {
            return false;
        };

        // First figure out if we are descending or ascending
        let is_descending = readings[1] < readings[0];

        // Then check if the readings don't change too fast
        readings.windows(2).all(|pair| {
            is_safe(pair[0], pair[1], is_descending)
        })
    }).count().try_into().map_err(|err: std::num::TryFromIntError| err.to_string())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    contents.lines().filter(|line| {
        let Ok(readings) = try_parse_readings(line) else {
            return false;
        };

        let num_negative = readings.windows(2).map(|pair| {
            pair[1] as i64 - pair[0] as i64
        }).filter(|&val| val < 0).count();
        let is_descending = num_negative > readings.len() - num_negative;

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
                return false;
            }
        }

        readings[1..].windows(3).all(|triplet| {
            let first_pair_is_safe = if index_to_remove >= 0 {
                true
            } else {
                is_safe(triplet[0], triplet[1], is_descending)
            };

            let second_pair_is_safe = if index_to_remove == 1 {
                is_safe(triplet[0], triplet[2], is_descending)
            } else {
                is_safe(triplet[1], triplet[2], is_descending)
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
                if is_safe(triplet[0], triplet[2], is_descending) {
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
        })
    }).count().try_into().map_err(|err: std::num::TryFromIntError| err.to_string())
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

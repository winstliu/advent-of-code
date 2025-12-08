pub fn part_1(contents: &str) -> Result<u64, String> {
    let ranges = contents.trim().split(',');
    let mut total: u64 = 0;
    for range in ranges {
        let (start, end) = range.split_once('-').ok_or("Range did not have a dash")?;
        let start = start.parse::<u64>().map_err(|e| e.to_string())?;
        let end = end.parse::<u64>().map_err(|e| e.to_string())?;

        let start_digits = start.ilog10() + 1;
        let end_digits = end.ilog10() + 1;

        // Easy case: range doesn't change # of digits, and there's an odd number of digits so we can't have a double
        if start_digits % 2 == 1 && start_digits == end_digits {
            continue;
        }

        let first_even_start_digit = if start_digits % 2 == 0 {
            start_digits
        } else {
            start_digits + 1
        };
        let last_even_end_digit = if end_digits % 2 == 0 {
            end_digits
        } else {
            end_digits - 1
        };

        for digits in (first_even_start_digit..=last_even_end_digit).step_by(2) {
            let first_double = 10_u64.pow(digits - 1) + 10_u64.pow(digits / 2 - 1);
            let diff_between_doubles = 10_u64.pow(digits / 2) + 1;

            let first_double_within_range = if start < first_double {
                first_double
            } else {
                first_double
                    + (start - first_double).div_ceil(diff_between_doubles) * diff_between_doubles
            };

            let end = if end < 10_u64.pow(digits) {
                end
            } else {
                10_u64.pow(digits) - 1
            };

            total += (first_double_within_range..=end)
                .step_by(diff_between_doubles as usize)
                .sum::<u64>();
        }
    }
    Ok(total)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    Ok(0)
}

pub fn part_1(contents: &str) -> Result<u64, String> {
    contents.lines().filter(|line| {
        let Ok(report) = try_parse_report(line) else {
            return false;
        };

        is_report_safe(&report)
    }).count().try_into().map_err(|err: std::num::TryFromIntError| err.to_string())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    contents.lines().filter(|line| {
        let Ok(report) = try_parse_report(line) else {
            return false;
        };

        // First see if the full report is safe
        if is_report_safe(&report) {
            return true;
        }

        // If not, can we remove one level to make it safe?
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);

            if is_report_safe(&new_report) {
                return true;
            }
        }

        false
    }).count().try_into().map_err(|err: std::num::TryFromIntError| err.to_string())
}

fn try_parse_report(line: &str) -> Result<Vec<u64>, String> {
    line.split_ascii_whitespace()
        .map(|val| val.parse::<u64>().map_err(|err| err.to_string()))
        .collect()
}

fn is_report_safe(report: &[u64]) -> bool {
    // First figure out if we are descending or ascending
    let is_descending = report[1] < report[0];

    // Then check if the levels don't change too fast
    report.windows(2).all(|pair| {
        if (is_descending && pair[1] >= pair[0]) || (!is_descending && pair[1] <= pair[0]) {
            return false;
        }
        pair[1].abs_diff(pair[0]) <= 3
    })
}

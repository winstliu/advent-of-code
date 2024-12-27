use regex::Regex;

pub fn part_1(contents: &str) -> Result<u64, String> {
    try_parse_equations(contents)?
        .iter_mut()
        .try_fold(0, |acc: u64, equations| {
            let Some((a, b)) = solve_equations(equations) else {
                return Ok(acc);
            };

            if a < 100 && b < 100 {
                return acc
                    .checked_add_signed(a * 3 + b)
                    .ok_or("Overflow".to_string());
            }

            Ok(acc)
        })
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    try_parse_equations(contents)?
        .iter_mut()
        .try_fold(0, |acc: u64, equations| {
            equations[0].2 += 10000000000000;
            equations[1].2 += 10000000000000;

            let Some((a, b)) = solve_equations(equations) else {
                return Ok(acc);
            };

            acc.checked_add_signed(a * 3 + b)
                .ok_or("Overflow".to_string())
        })
}

fn solve_equations(equations: &mut [Equation; 2]) -> Option<(i64, i64)> {
    let initial_a_coefficient = equations[0].0;

    // Set the equations to a common multiple to make division easy
    equations[0].0 *= equations[1].0;
    equations[0].1 *= equations[1].0;
    equations[0].2 *= equations[1].0;
    // 3196a + 748b = 285600

    equations[1].0 *= initial_a_coefficient;
    equations[1].1 *= initial_a_coefficient;
    equations[1].2 *= initial_a_coefficient;
    // 3196a + 6298b = 507600

    // Cancel out a in equations[0]
    equations[0].1 -= equations[1].1;
    equations[0].2 -= equations[1].2;
    // (748 - 6298)b = 285600 - 507600

    // Solve for b using equations[0]
    let b = equations[0].2 / equations[0].1;
    if equations[0].2 % equations[0].1 != 0 {
        return None;
    }

    // Solve for a using equations[1]
    equations[1].2 -= equations[1].1 * b;
    let a = equations[1].2 / equations[1].0;
    if equations[1].2 % equations[1].0 != 0 {
        return None;
    }

    Some((a, b))
}

fn try_parse_equations(contents: &str) -> Result<Vec<[Equation; 2]>, String> {
    let regex = Regex::new(r"(\d+), [^\d]+?(\d+)$").map_err(|err| err.to_string())?;
    contents
        .split("\r\n\r\n")
        .map(|equations| {
            let equations = equations
                .trim()
                .split("\r\n")
                .map(|equation| {
                    let captures = regex
                        .captures(equation)
                        .ok_or(format!("No match: {}", equation).to_string())?;

                    let a = captures
                        .get(1)
                        .ok_or("No first capture group")?
                        .as_str()
                        .parse::<i64>()
                        .map_err(|err| err.to_string())?;
                    let b = captures
                        .get(2)
                        .ok_or("No second capture group")?
                        .as_str()
                        .parse::<i64>()
                        .map_err(|err| err.to_string())?;

                    Ok((a, b))
                })
                .collect::<Result<Vec<_>, String>>()?;

            let a = equations[0];
            let b = equations[1];
            let prize = equations[2];

            Ok([(a.0, b.0, prize.0), (a.1, b.1, prize.1)])
        })
        .collect()
}

type Equation = (i64, i64, i64);

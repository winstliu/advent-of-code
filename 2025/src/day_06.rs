use std::ops::Add;
use std::ops::Mul;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let operands_per_equation = contents.lines().count() - 1;
    let num_equations = contents
        .lines()
        .next()
        .ok_or("Expected at least one operand".to_string())?
        .split_ascii_whitespace()
        .count();
    let mut operands = vec![Vec::new(); num_equations];

    for line in contents.lines().take(operands_per_equation) {
        for (i, operand) in line.split_ascii_whitespace().enumerate() {
            operands[i].push(operand.parse::<u64>().map_err(|err| err.to_string())?)
        }
    }

    let operators = contents
        .lines()
        .nth(operands_per_equation)
        .ok_or("Expected operators line".to_string())?
        .split_ascii_whitespace()
        .map::<Result<fn(u64, u64) -> _, _>, _>(|operator| match operator {
            "+" => Ok(u64::add),
            "*" => Ok(u64::mul),
            _ => Err("Unexpected operator".to_string()),
        })
        .collect::<Result<Vec<_>, _>>()?;

    operands
        .into_iter()
        .zip(operators)
        .try_fold(0, |acc, (operands, operator)| {
            Ok(acc
                + operands
                    .into_iter()
                    .reduce(&operator)
                    .ok_or("Expected at least one operand".to_string())?)
        })
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    Ok(0)
}

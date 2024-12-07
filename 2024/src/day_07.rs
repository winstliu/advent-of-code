use std::ops::Add;
use std::ops::Mul;

use itertools::Itertools;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let operators: [fn(u64, u64) -> _; 2] = [u64::add, u64::mul];

    Ok(contents
        .lines()
        .filter_map(|line| {
            let Ok((test_value, numbers)) = try_parse_equation(line) else {
                return None;
            };

            itertools::repeat_n(operators, numbers.len() - 1)
                .multi_cartesian_product()
                .any(|ops| {
                    numbers
                        .iter()
                        .enumerate()
                        .skip(1)
                        .fold(numbers[0], |acc, (i, x)| ops[i - 1](acc, *x))
                        == test_value
                })
                .then_some(test_value)
        })
        .sum())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let operators: [fn(u64, u64) -> _; 3] = [u64::add, u64::mul, concat];

    Ok(contents
        .lines()
        .filter_map(|line| {
            let Ok((test_value, numbers)) = try_parse_equation(line) else {
                return None;
            };

            itertools::repeat_n(operators, numbers.len() - 1)
                .multi_cartesian_product()
                .any(|ops| {
                    numbers
                        .iter()
                        .enumerate()
                        .skip(1)
                        .fold(numbers[0], |acc, (i, x)| ops[i - 1](acc, *x))
                        == test_value
                })
                .then_some(test_value)
        })
        .sum())
}

fn try_parse_equation(line: &str) -> Result<(u64, Vec<u64>), String> {
    let mut line = line.split(':');
    let test_value = line
        .next()
        .ok_or("No test value")?
        .parse::<u64>()
        .map_err(|err| err.to_string())?;
    let numbers = line
        .next()
        .ok_or("No numbers")?
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().map_err(|err| err.to_string()))
        .collect::<Result<_, _>>()?;

    Ok((test_value, numbers))
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

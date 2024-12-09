use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let (antenna_locations, (row_length, col_length)) = parse_grid(contents);
    antenna_locations
        .values()
        .fold(HashSet::new(), |mut acc, locations| {
            for (a, b) in locations.iter().tuple_combinations() {
                let (row_distance, col_distance) =
                    (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);

                if let (Some(row), Some(col)) = (
                    a.0.checked_add_signed(-row_distance),
                    a.1.checked_add_signed(-col_distance),
                ) {
                    if row < row_length && col < col_length {
                        acc.insert((row, col));
                    }
                }

                if let (Some(row), Some(col)) = (
                    b.0.checked_add_signed(row_distance),
                    b.1.checked_add_signed(col_distance),
                ) {
                    if row < row_length && col < col_length {
                        acc.insert((row, col));
                    }
                }
            }

            acc
        })
        .len()
        .try_into()
        .map_err(|err: std::num::TryFromIntError| err.to_string())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let (antenna_locations, (row_length, col_length)) = parse_grid(contents);
    antenna_locations
        .values()
        .fold(HashSet::new(), |mut acc, locations| {
            for (a, b) in locations.iter().tuple_combinations() {
                let (row_distance, col_distance) =
                    (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);

                let mut antinode_position = *a;
                while let (Some(row), Some(col)) = (
                    antinode_position.0.checked_add_signed(row_distance),
                    antinode_position.1.checked_add_signed(col_distance),
                ) {
                    if row < row_length && col < col_length {
                        acc.insert((row, col));
                    } else {
                        break;
                    }

                    antinode_position = (row, col);
                }

                let mut antinode_position = *b;
                while let (Some(row), Some(col)) = (
                    antinode_position.0.checked_add_signed(-row_distance),
                    antinode_position.1.checked_add_signed(-col_distance),
                ) {
                    if row < row_length && col < col_length {
                        acc.insert((row, col));
                    } else {
                        break;
                    }

                    antinode_position = (row, col);
                }
            }

            acc
        })
        .len()
        .try_into()
        .map_err(|err: std::num::TryFromIntError| err.to_string())
}

fn parse_grid(contents: &str) -> (HashMap<char, Vec<Position>>, (usize, usize)) {
    let mut row_length = 0;
    let mut col_length = 0;
    (
        contents
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row, line)| {
                col_length = 0;

                for (col, c) in line.chars().enumerate() {
                    col_length += 1;

                    if !c.is_ascii_alphanumeric() {
                        continue;
                    }

                    acc.entry(c)
                        .and_modify(|v| v.push((row, col)))
                        .or_insert(vec![(row, col)]);
                }

                row_length += 1;

                acc
            }),
        (row_length, col_length),
    )
}

type Position = (usize, usize);

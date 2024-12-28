pub fn part_1(contents: &str) -> Result<u64, String> {
    let max_x = 100;
    let max_y = 102;
    Ok(try_parse_input(contents)?
        .iter()
        .fold([0; 4], |mut acc, ((x, y), (dx, dy))| {
            let new_x = (*x as i64 + dx * 100).rem_euclid(max_x + 1);
            let new_y = (*y as i64 + dy * 100).rem_euclid(max_y + 1);

            if new_x < max_x / 2 && new_y < max_y / 2 {
                acc[0] += 1;
            } else if new_x < max_x / 2 && new_y > max_y / 2 {
                acc[1] += 1;
            } else if new_x > max_x / 2 && new_y < max_y / 2 {
                acc[2] += 1;
            } else if new_x > max_x / 2 && new_y > max_y / 2 {
                acc[3] += 1;
            }

            acc
        })
        .iter()
        .product())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    Ok(0)
}

fn try_parse_input(contents: &str) -> Result<Vec<(Position, Velocity)>, String> {
    contents
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let Some(position) = parts.next() else {
                return Err(format!("No position: {}", line));
            };

            let Some(velocity) = parts.next() else {
                return Err(format!("No velocity: {}", line));
            };

            let position = position
                .trim_start_matches("p=")
                .split(',')
                .map(|part| part.parse::<u64>().map_err(|err| err.to_string()))
                .collect::<Result<Vec<_>, _>>()?;

            let velocity = velocity
                .trim_start_matches("v=")
                .split(',')
                .map(|part| part.parse::<i64>().map_err(|err| err.to_string()))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(((position[0], position[1]), (velocity[0], velocity[1])))
        })
        .collect()
}

type Position = (u64, u64);
type Velocity = (i64, i64);

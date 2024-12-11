
pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut stones = try_get_stones(contents)?;
    for _ in 0..25 {
        stones = stones.iter().flat_map(mutate_stone).collect();
    }
    stones.len().try_into().map_err(|err: std::num::TryFromIntError| err.to_string())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut stones = try_get_stones(contents)?;
    for _ in 0..75 {
        stones = stones.iter().flat_map(mutate_stone).collect();
    }
    stones.len().try_into().map_err(|err: std::num::TryFromIntError| err.to_string())
}

fn try_get_stones(contents: &str) -> Result<Vec<u64>, String> {
    contents.split_ascii_whitespace().map(|stone| {
        stone.parse::<u64>().map_err(|err| err.to_string())
    }).collect()
}

fn mutate_stone(stone: &u64) -> Vec<u64> {
    if *stone == 0 {
        return vec![1];
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        return vec![stone / (10_u64.pow(digits / 2)), stone % (10_u64.pow(digits / 2))];
    }

    vec![stone * 2024]
}

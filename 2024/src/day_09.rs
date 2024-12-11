use std::cmp::min;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let file_blocks = try_parse_input(contents)?;
    let mut occupied_blocks = file_blocks
        .iter()
        .filter_map(|block| block.0.map(|id| (id, block.1, block.2)))
        .collect::<Vec<_>>();

    let mut acc = 0;
    for block in file_blocks.iter() {
        if let Some(id) = block.0 {
            let mut length = block.2;
            let last_occupied_block = occupied_blocks[occupied_blocks.len() - 1];
            if last_occupied_block.0 == id {
                length = last_occupied_block.2;
            }

            let id: u64 = id
                .try_into()
                .map_err(|err: std::num::TryFromIntError| err.to_string())?;
            let sum: u64 = (block.1..(block.1 + length)).sum();
            acc += id * sum;
        } else {
            let mut remaining_free_length = block.2;
            let mut done = false;
            while remaining_free_length > 0 {
                let Some(last_occupied_block) = occupied_blocks.pop() else {
                    break;
                };

                if last_occupied_block.1 <= block.1 {
                    done = true;
                    break;
                }

                let transfer_length = min(last_occupied_block.2, remaining_free_length);

                let id: u64 = last_occupied_block
                    .0
                    .try_into()
                    .map_err(|err: std::num::TryFromIntError| err.to_string())?;
                let start = block.1 + block.2 - remaining_free_length;
                let sum: u64 = (start..(start + transfer_length)).sum();
                acc += id * sum;

                if transfer_length < last_occupied_block.2 {
                    occupied_blocks.push((
                        last_occupied_block.0,
                        last_occupied_block.1,
                        last_occupied_block.2 - transfer_length,
                    ));
                }

                remaining_free_length -= transfer_length;
            }

            if done {
                break;
            }
        }
    }

    Ok(acc)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let file_blocks = try_parse_input(contents)?;
    let occupied_blocks = file_blocks
        .iter()
        .filter_map(|block| block.0.map(|id| (id, block.1, block.2)))
        .collect::<Vec<_>>();
    let mut empty_blocks = file_blocks
        .into_iter()
        .filter(|block| block.0.is_none())
        .collect::<Vec<_>>();

    let mut acc = 0;
    for occupied_block in occupied_blocks.iter().rev() {
        let mut moved = false;
        for empty_block in empty_blocks.iter_mut() {
            if empty_block.1 > occupied_block.1 {
                break;
            }

            if occupied_block.2 <= empty_block.2 {
                let transfer_length = occupied_block.2;
                let id: u64 = occupied_block
                    .0
                    .try_into()
                    .map_err(|err: std::num::TryFromIntError| err.to_string())?;
                let sum: u64 = (empty_block.1..(empty_block.1 + transfer_length)).sum();
                acc += id * sum;

                *empty_block = (
                    None,
                    empty_block.1 + transfer_length,
                    empty_block.2 - transfer_length,
                );

                moved = true;
                break;
            }
        }

        if !moved {
            let id: u64 = occupied_block
                .0
                .try_into()
                .map_err(|err: std::num::TryFromIntError| err.to_string())?;
            let sum: u64 = (occupied_block.1..(occupied_block.1 + occupied_block.2)).sum();
            acc += id * sum;
        }
    }

    Ok(acc)
}

// id, start, length
fn try_parse_input(contents: &str) -> Result<Vec<(Option<usize>, u64, u64)>, String> {
    let mut start = 0;
    contents
        .trim()
        .chars()
        .enumerate()
        .map(|(i, len)| {
            let Some(len) = len.to_digit(10).map(u64::from) else {
                return Err(format!("could not parse {} as u64", len));
            };

            let res = if i % 2 == 0 {
                Ok((Some(i / 2), start, len))
            } else {
                Ok((None, start, len))
            };

            start += len;
            res
        })
        .collect()
}

use std::collections::HashSet;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut rows = contents.lines();
    let first_row = rows.next().ok_or("Expected at least one row")?;

    let mut beam_columns = HashSet::new();
    beam_columns.insert(first_row.find('S').ok_or("Expected starting location")?);

    let mut total_split = 0;
    for row in rows {
        let splitters = row.match_indices('^').map(|(column, _)| column);
        for splitter in splitters {
            if beam_columns.contains(&splitter) {
                total_split += 1;
                beam_columns.remove(&splitter);
                beam_columns.insert(splitter - 1);
                beam_columns.insert(splitter + 1);
            }
        }
    }

    Ok(total_split)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut rows = contents.lines();
    let first_row = rows.next().ok_or("Expected at least one row")?;

    let mut beam_columns = HashSet::new();
    beam_columns.insert(first_row.find('S').ok_or("Expected starting location")?);

    let mut timelines = 0;
    for row in rows {
        let splitters = row.match_indices('^').map(|(column, _)| column);
        let existing_beams = beam_columns.clone();
        for splitter in splitters {
            if beam_columns.contains(&splitter) {
                beam_columns.remove(&splitter);
                beam_columns.insert(splitter - 1);
                beam_columns.insert(splitter + 1);
            }
        }

        if beam_columns != existing_beams {
            timelines += beam_columns.len();
        }
    }

    timelines
        .try_into()
        .map_err(|err: std::num::TryFromIntError| err.to_string())
}

// fn parse_manifold(contents: &str) -> Vec<Vec<char>> {

// }

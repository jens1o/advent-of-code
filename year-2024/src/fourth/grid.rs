#[derive(Debug)]
pub(super) struct Grid {
    pub(super) length_per_line: usize,
    pub(super) number_of_lines: usize,
    pub(super) grid_structure: Vec<Vec<char>>,
}

pub(super) fn prepare_grid(payload: impl AsRef<str>) -> Grid {
    let payload = payload.as_ref();

    let line_count = payload.lines().count();
    assert!(line_count > 0);
    assert!(are_line_lengths_equal(payload));

    let line_length = payload.lines().next().unwrap().len();

    let mut grid_structure: Vec<Vec<char>> = Vec::with_capacity(line_count);

    for line in payload.lines() {
        let mut line_buffer = Vec::with_capacity(line_length);

        for character in line.chars() {
            line_buffer.push(character);
        }

        grid_structure.push(line_buffer);
    }

    Grid {
        length_per_line: line_length,
        number_of_lines: line_count,
        grid_structure: grid_structure,
    }
}

fn are_line_lengths_equal(payload: impl AsRef<str>) -> bool {
    let mut previously_seen_line_length = None;
    let payload = payload.as_ref();

    for current_line_length in payload.lines().map(|x| x.len()) {
        match previously_seen_line_length {
            Some(len) if len != current_line_length => return false,
            _ => previously_seen_line_length = Some(current_line_length),
        }
    }

    if previously_seen_line_length.is_none() {
        panic!("payload is empty?");
    }

    true
}

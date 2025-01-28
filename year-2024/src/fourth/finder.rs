//! Searches for occurences of XMAS in a given grid and counts the occurences.
//! (Occurences can be horizontal, vertical, diagonal, written backwards, or even overlapping other words)

use super::grid::Grid;

const NEEDLE: &str = "XMAS";

pub(super) fn count_xmas(grid: &Grid) -> usize {
    let mut found_counter = 0;

    assert!(grid.length_per_line >= NEEDLE.len());

    found_counter += find_horizontals(grid);
    found_counter += find_verticals(grid);
    found_counter += find_diagonals(grid);

    found_counter
}

fn find_diagonals(grid: &Grid) -> usize {
    let mut found_counter = 0;

    // iterate through each line aka row, look for an X (start of XMAS) and then search diagonal in four directions from the finding (above left/right, below left/right)
    for (row_no, line) in grid.grid_structure.iter().enumerate() {
        for (col_no, _found_x) in line
            .iter()
            .enumerate()
            .filter(|(_, character)| **character == 'X')
        {
            // check if it is possible that we still have enough room to look diagonally in the four possible cases:

            // XMAS is above left, e.g.
            // ----
            // Sxxx
            // xAxx
            // xxMx
            // xxxX <- X found here
            // ---
            // => we need at least three other rows above the finding and the column we're in must be in at least fourth place
            if row_no >= 3 && col_no >= 3 {
                // check above left
                if (&grid.grid_structure)[row_no - 3][col_no - 3] == 'S'
                    && (&grid.grid_structure)[row_no - 2][col_no - 2] == 'A'
                    && (&grid.grid_structure)[row_no - 1][col_no - 1] == 'M'
                {
                    found_counter += 1;
                }
            }

            // XMAS is above right, e.g.
            // ----
            // xxxS
            // xxAx
            // xMxx
            // Xxxx <- X found here
            // ----
            // => we need at least three other rows above the finding and the column we're in must be at most in the fourth last place
            if row_no >= 3 && col_no + 3 < grid.length_per_line {
                // check above right
                if (&grid.grid_structure)[row_no - 3][col_no + 3] == 'S'
                    && (&grid.grid_structure)[row_no - 2][col_no + 2] == 'A'
                    && (&grid.grid_structure)[row_no - 1][col_no + 1] == 'M'
                {
                    found_counter += 1;
                }
            }

            // XMAS is below left, e.g.
            // ----
            // xxxX <- X found here
            // xxMx
            // xAxx
            // Sxxx
            // ----
            // => we need at least three more rows below us and we must be in the at least fourth column
            if row_no + 3 < grid.number_of_lines && col_no >= 3 {
                // check above right
                if (&grid.grid_structure)[row_no + 1][col_no - 1] == 'M'
                    && (&grid.grid_structure)[row_no + 2][col_no - 2] == 'A'
                    && (&grid.grid_structure)[row_no + 3][col_no - 3] == 'S'
                {
                    found_counter += 1;
                }
            }

            // XMAS is below right, e.g.
            // ----
            // Xxxx <- X found here
            // xMxx
            // xxAx
            // xxxS
            // ----
            // => we need at least three more columns right to us and the row we're in must be at most in the fourth last place
            if col_no + 3 < grid.length_per_line && row_no + 3 < grid.number_of_lines {
                // check above right
                if (&grid.grid_structure)[row_no + 1][col_no + 1] == 'M'
                    && (&grid.grid_structure)[row_no + 2][col_no + 2] == 'A'
                    && (&grid.grid_structure)[row_no + 3][col_no + 3] == 'S'
                {
                    found_counter += 1;
                }
            }
        }
    }

    found_counter
}

fn find_verticals(grid: &Grid) -> usize {
    let mut found_counter = 0;

    for column_no in 0..grid.length_per_line {
        // read out current column in the lines and transform it to a horizontal line
        let column_text: Vec<char> = grid
            .grid_structure
            .iter()
            .map(|line| line[column_no])
            .collect();

        found_counter += count_horizontal_in_line(&column_text);
    }

    found_counter
}

fn find_horizontals(grid: &Grid) -> usize {
    let mut found_counter = 0;

    assert!(grid.length_per_line >= NEEDLE.len());

    for line in &grid.grid_structure {
        found_counter += count_horizontal_in_line(line);
    }

    found_counter
}

fn count_horizontal_in_line(line: &[char]) -> usize {
    let mut counter = 0;

    for i in 0..=(line.len() - NEEDLE.len()) {
        if line[i] == 'X' && line[i + 1] == 'M' && line[i + 2] == 'A' && line[i + 3] == 'S' {
            // read forwards
            counter += 1;
        } else if line[i] == 'S' && line[i + 1] == 'A' && line[i + 2] == 'M' && line[i + 3] == 'X' {
            // read backwards
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::fourth::{finder::find_diagonals, grid::Grid};

    use super::{count_horizontal_in_line, find_verticals};

    #[test]
    fn test_horizontal_count() {
        let haystack = vec!['X', 'M', 'A', 'S'];

        assert_eq!(count_horizontal_in_line(&haystack), 1);

        let haystack = vec!['.', 'S', 'A', 'M', 'X', 'M', 'A', 'S'];
        // read forwards and backwards
        assert_eq!(count_horizontal_in_line(&haystack), 2);
    }

    #[test]
    fn test_vertical_count() {
        let grid = Grid {
            length_per_line: 2,
            number_of_lines: 4,
            grid_structure: vec![
                vec!['X', 'S'],
                vec!['M', 'A'],
                vec!['A', 'M'],
                vec!['S', 'X'],
            ],
        };

        assert_eq!(find_verticals(&grid), 2);
    }

    #[test]
    fn test_diagonal_count() {
        let grid = Grid {
            length_per_line: 4,
            number_of_lines: 4,
            grid_structure: vec![
                vec!['x', 'x', 'x', 'S'],
                vec!['x', 'x', 'A', 'x'],
                vec!['x', 'M', 'x', 'x'],
                vec!['X', 'x', 'x', 'x'],
            ],
        };

        assert_eq!(find_diagonals(&grid), 1);
    }
}

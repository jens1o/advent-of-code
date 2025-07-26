use super::grid::Grid;

pub(super) fn count_crossmas(grid: &Grid) -> usize {
    let mut found_crossmas = 0;

    // Strategy: Look for an A in the middle and then check diagonally

    for (row_no, line) in grid.grid_structure.iter().enumerate() {
        for (col_no, _character) in line.iter().enumerate().filter(|(_, cand)| **cand == 'A') {
            if col_no > 0
                && row_no > 0
                && col_no + 1 < grid.length_per_line
                && row_no + 1 < grid.number_of_lines
                && ((grid.grid_structure[row_no - 1][col_no - 1] == 'M'
                    && grid.grid_structure[row_no + 1][col_no + 1] == 'S')
                    || (grid.grid_structure[row_no - 1][col_no - 1] == 'S'
                        && grid.grid_structure[row_no + 1][col_no + 1] == 'M'))
                    && ((grid.grid_structure[row_no + 1][col_no - 1] == 'M'
                        && grid.grid_structure[row_no - 1][col_no + 1] == 'S')
                        || (grid.grid_structure[row_no + 1][col_no - 1] == 'S'
                            && grid.grid_structure[row_no - 1][col_no + 1] == 'M'))
                {
                    found_crossmas += 1;
                }
        }
    }

    found_crossmas
}

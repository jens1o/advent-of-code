use crossmas_finder::count_crossmas;
use xmas_finder::count_xmas;

mod crossmas_finder;
mod grid;
mod xmas_finder;

#[allow(unused)]
const SAMPLE_PAYLOAD_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[allow(unused)]
const SAMPLE_PAYLOAD_2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

#[allow(unused)]
pub fn fourth_december() {
    // let payload = SAMPLE_PAYLOAD_2;
    let payload = include_str!("fourth.txt");

    let grid = grid::prepare_grid(payload);

    let occurrences = &count_xmas(&grid);
    dbg!(occurrences);

    let occurrences = &count_crossmas(&grid);
    dbg!(occurrences);
}

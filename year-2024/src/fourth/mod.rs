use std::time::Instant;

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
    let payload = include_str!("fourth.txt");
    // let payload = SAMPLE_PAYLOAD_2;

    let now = Instant::now();

    let grid = grid::prepare_grid(payload);
    dbg!(now.elapsed());

    /*let now = Instant::now();
    // dbg!(&grid);

    let occurences = &count_xmas(&grid);
    dbg!(now.elapsed());

    dbg!(occurences);*/

    let now = Instant::now();

    let occurences = &count_crossmas(&grid);
    dbg!(now.elapsed());
    dbg!(occurences);
}

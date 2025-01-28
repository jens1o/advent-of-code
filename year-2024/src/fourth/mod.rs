use std::time::Instant;

use finder::count_xmas;

mod finder;
mod grid;

#[allow(unused)]
const SAMPLE_PAYLOAD: &str = "MMMSXXMASM
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
pub fn fourth_december() {
    let payload = include_str!("fourth.txt");

    let now = Instant::now();

    let grid = grid::prepare_grid(payload);
    // dbg!(&grid);

    let occurences = &count_xmas(&grid);
    dbg!(now.elapsed());
    dbg!(occurences);
}

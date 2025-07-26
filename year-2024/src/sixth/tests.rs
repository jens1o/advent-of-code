use crate::sixth::{parse_puzzle_input, part_1, part_2};

const SAMPLE: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_part_2_sample() {
    let input = SAMPLE.trim();

    let (map, guard) = parse_puzzle_input(input);

    let result = part_2(map, guard);

    assert_eq!(result, [(1, 8), (3, 6), (3, 8), (6, 7), (7, 7), (7, 9)]);
}

#[test]
fn test_part_2_subreddit_help() {
    const CORNER_CASE: &str = "
....
#...
.^#.
.#..";

    let (map, guard) = parse_puzzle_input(CORNER_CASE.trim());

    println!("{}", map.text_representation(Some(&guard)));

    assert_eq!(part_1(&map, guard.clone()), 3);
    assert!(part_2(map, guard).is_empty());
}

#[test]
fn test_part_2_other_example() {
    const CASE: &str = "
.##..
....#
#..#.
.^#..";

    let (map, guard) = parse_puzzle_input(CASE.trim());

    println!("{}", map.text_representation(Some(&guard)));

    assert_eq!(part_2(map, guard), [(0, 1), (1, 1), (3, 1)]);
}

#[test]
fn test_part_2_another() {
    const CASE: &str = "
..#..
....#
#..#.
.^...
.#...
..#..";

    let (map, guard) = parse_puzzle_input(CASE.trim());

    println!("{}", map.text_representation(Some(&guard)));

    assert_eq!(part_2(map, guard).len(), 1);
}

#[test]
fn test_part_2_yet_another_debug_map() {
    const CASE: &str = "
...........#.....#......
...................#....
...#.....##.............
......................#.
..................#.....
..#.....................
....................#...
........................
.#........^.............
..........#..........#..
..#.....#..........#....
........#.....#..#......";

    let (map, guard) = parse_puzzle_input(CASE.trim());

    println!("{}", map.text_representation(Some(&guard)));

    assert_eq!(part_2(map, guard).len(), 19);
}

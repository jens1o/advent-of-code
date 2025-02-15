use core::panic;
use std::collections::HashSet;

const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[derive(Debug, Clone, Copy)]
enum Direction {
    Upwards,
    Right,
    Downwards,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        use Direction::*;

        match &self {
            Upwards => Right,
            Right => Downwards,
            Downwards => Left,
            Left => Upwards,
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: (usize, usize),
    current_direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapTile {
    Obstruction,
    Free,
}

#[derive(Debug)]
struct Map {
    size: (usize, usize),
    map: Vec<Vec<MapTile>>,
}

impl Map {
    /// Finds the next obstruction based on the current position.
    /// Returns the position that is exactly one step away before running into the obstruction.
    /// Returns None when you fall outside the world (there is no obstruction on the way to void).
    pub fn get_next_obstruction(
        &self,
        start_point: (usize, usize),
        direction: Direction,
    ) -> (Option<(usize, usize)>, Vec<(usize, usize)>) {
        let mut visited_positions = vec![start_point];

        loop {
            let current_position = visited_positions[visited_positions.len() - 1];

            let next_tile = match direction {
                Direction::Upwards => {
                    if current_position.1 == 0 {
                        // would fall out of world
                        return (None, visited_positions);
                    }

                    (current_position.0, current_position.1 - 1)
                }
                Direction::Right => {
                    if current_position.0 == self.size.0 - 1 {
                        // would fall out of world
                        return (None, visited_positions);
                    }
                    (current_position.0 + 1, current_position.1)
                }
                Direction::Downwards => {
                    if current_position.1 == self.size.1 - 1 {
                        // would fall out of world
                        return (None, visited_positions);
                    }

                    (current_position.0, current_position.1 + 1)
                }
                Direction::Left => {
                    if current_position.0 == 0 {
                        // would fall out of world
                        return (None, visited_positions);
                    }
                    (current_position.0 - 1, current_position.1)
                }
            };

            let next_map_tile = self.map[next_tile.1][next_tile.0];
            if next_map_tile == MapTile::Obstruction {
                return (Some(current_position), visited_positions);
            }

            visited_positions.push(next_tile);
        }
    }
}

pub(crate) fn sixth_december() {
    // let input = SAMPLE;
    let input = include_str!("sixth.txt");

    let (map, mut guard) = parse_puzzle_input(input);
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

    loop {
        match map.get_next_obstruction(guard.position, guard.current_direction) {
            (Some(new_position), walked_path) => {
                // mark way up to new_position as visited
                visited_positions.extend(walked_path);

                guard.position = new_position;
                guard.current_direction = guard.current_direction.turn_right();
            }
            (None, walked_path) => {
                // mark way up to next_obstruction as visited
                visited_positions.extend(walked_path);

                dbg!(visited_positions.len());
                break;
            }
        }
    }
}

fn parse_puzzle_input(input: impl AsRef<str>) -> (Map, Guard) {
    let input = input.as_ref();

    let mut size = (None, 0);
    let mut guard_position = None;

    let mut map = Vec::new();

    for (y, line) in input.lines().enumerate() {
        match size.0 {
            None => size.0 = Some(line.len()),
            Some(x) => {
                if x != line.len() {
                    panic!("map is not square");
                }
            }
        }

        size.1 += 1;

        let mut row = Vec::new();
        for (x, map_tile) in line.chars().enumerate() {
            match map_tile {
                '^' => {
                    guard_position = Some((x, y));
                }
                '#' => {
                    row.push(MapTile::Obstruction);
                }
                _ => {
                    row.push(MapTile::Free);
                }
            }
        }

        map.push(row);
    }

    (
        Map {
            size: (size.0.unwrap(), size.1),
            map,
        },
        Guard {
            current_direction: Direction::Upwards,
            position: guard_position.unwrap(),
        },
    )
}

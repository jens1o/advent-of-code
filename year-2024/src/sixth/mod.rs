use core::{fmt, panic};
use std::collections::HashSet;

#[cfg(test)]
mod tests;

#[allow(unused)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Upwards,
    Right,
    Downwards,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;

        let display_representation = match self {
            Upwards => '⇑',
            Right => '⇒',
            Downwards => '⇓',
            Left => '⇐',
        };

        write!(f, "{display_representation}")
    }
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

#[derive(Debug, Clone)]
struct Guard {
    position: (usize, usize),
    current_direction: Direction,
}

#[derive(Clone, Copy, PartialEq)]
enum MapTile {
    Obstruction,
    HighlightedObstruction,
    Free,
}

impl fmt::Debug for MapTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Obstruction => write!(f, "#"),
            Self::HighlightedObstruction => write!(f, "$"),
            Self::Free => write!(f, "."),
        }
    }
}

#[derive(Clone)]
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
            if next_map_tile == MapTile::Obstruction
                || next_map_tile == MapTile::HighlightedObstruction
            {
                return (Some(current_position), visited_positions);
            }

            visited_positions.push(next_tile);
        }
    }

    pub fn text_representation(&self, guard: Option<&Guard>) -> String {
        let mut map_representation = String::new();

        for (y, rows) in self.map.iter().enumerate() {
            map_representation.push_str(&format!("{y:03} "));

            for (x, tile) in rows.iter().enumerate() {
                if let Some(guard) = guard {
                    if guard.position == (x, y) {
                        map_representation.push_str(&format!("{}", guard.current_direction));
                        continue;
                    }
                }

                map_representation.push_str(&format!("{tile:?}"));
            }

            map_representation.push('\n');
        }

        map_representation
    }
}

pub(crate) fn sixth_december() {
    // let input = SAMPLE.trim();
    let input = include_str!("sixth.txt");

    let (map, guard) = parse_puzzle_input(input);

    println!("{}", map.text_representation(Some(&guard)));

    dbg!(part_1(&map, guard.clone()));

    let locations_for_looping_the_security = part_2(map, guard);

    dbg!(locations_for_looping_the_security.len());
}

// Checks whether it is allowed to place an obstacle at the candidate's position
fn is_valid_position_for_obstacle((x, y): (usize, usize), map: &Map, guard: &Guard) -> bool {
    // we can't place any obstacle at the position of the guard
    if guard.position == (x, y) {
        return false;
    }

    if map.size.0 <= x || map.size.1 <= y {
        dbg!(map.size, (x, y));
        return false;
    }

    // we can't place an additional obstacle where there is already one
    if map.map[y][x] == MapTile::Obstruction {
        return false;
    }

    true
}

fn part_2(map: Map, guard: Guard) -> Vec<(usize, usize)> {
    let mut possible_obstacle_positions_to_create_loops = Vec::new();

    for x in 0..map.size.0 {
        'outer: for y in 0..map.size.1 {
            if !is_valid_position_for_obstacle((x, y), &map, &guard) {
                continue;
            }

            let mut map_with_obstruction = map.clone();

            // place an obstacle and check whether we run into a loop
            map_with_obstruction.map[y][x] = MapTile::HighlightedObstruction;

            let mut guard_on_new_map = guard.clone();

            let mut positions_with_took_turns: HashSet<((usize, usize), Direction)> =
                HashSet::new();

            loop {
                match map_with_obstruction.get_next_obstruction(
                    guard_on_new_map.position,
                    guard_on_new_map.current_direction,
                ) {
                    (Some(new_position), _walked_path) => {
                        let in_loop = !positions_with_took_turns
                            .insert((new_position, guard_on_new_map.current_direction));

                        if in_loop {
                            // println!(
                            //     "found new position for obstacle at ({x}, {y}, guard_pos={:?}):",
                            //     guard.position
                            // );
                            // println!("{}", map_with_obstruction.text_representation(Some(&guard)));
                            // println!();

                            possible_obstacle_positions_to_create_loops.push((x, y));
                            continue 'outer;
                        }

                        guard_on_new_map.position = new_position;
                        guard_on_new_map.current_direction =
                            guard_on_new_map.current_direction.turn_right();
                    }
                    (None, walked_path) => {
                        guard_on_new_map.position = *walked_path.last().unwrap();

                        // we're not in a loop as we run out of the world
                        continue 'outer;
                    }
                }
            }
        }
    }

    possible_obstacle_positions_to_create_loops
}

fn part_1(map: &Map, mut guard: Guard) -> usize {
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

                return visited_positions.len();
            }
        }
    }
}

fn parse_puzzle_input(input: impl AsRef<str>) -> (Map, Guard) {
    let input = input.as_ref().trim();

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

        let mut row = Vec::with_capacity(line.len());
        for (x, map_tile) in line.chars().enumerate() {
            match map_tile {
                '^' => {
                    guard_position = Some((x, y));
                    row.push(MapTile::Free);
                }
                '#' => {
                    row.push(MapTile::Obstruction);
                }
                '.' => {
                    row.push(MapTile::Free);
                }
                _other => {
                    unreachable!();
                }
            }
        }

        debug_assert_eq!(row.len(), line.len());

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

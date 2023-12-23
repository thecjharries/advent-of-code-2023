// Copyright 2023 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    NorthDownSlope,
    EastDownSlope,
    SouthDownSlope,
    WestDownSlope,
}

impl Tile {
    fn from_char(character: char) -> Self {
        match character {
            '.' => Self::Empty,
            '>' => Self::EastDownSlope,
            '<' => Self::WestDownSlope,
            '^' => Self::NorthDownSlope,
            'v' => Self::SouthDownSlope,
            _ => Self::Wall,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    entrance: (usize, usize),
    exit: (usize, usize),
}

impl Map {
    fn from_string(input: String) -> Self {
        let input = input.trim();
        let mut lines = input.lines().collect::<Vec<&str>>();
        let mut tiles = Vec::new();
        let mut entrance = (0, 0);
        let mut exit = (0, 0);
        for (y, line) in lines.iter().enumerate() {
            let line = line.trim();
            let mut row = Vec::new();
            for (x, character) in line.chars().enumerate() {
                let tile = Tile::from_char(character);
                if 0 == y && '.' == character {
                    entrance = (x, y);
                }
                if lines.len() - 1 == y && '.' == character {
                    exit = (x, y);
                }
                row.push(tile);
            }
            tiles.push(row);
        }
        Self {
            tiles,
            entrance,
            exit,
        }
    }

    fn find_longest_path(&self) -> usize {
        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        queue.push((0, self.entrance.0, self.entrance.1));
        while let Some((steps, x, y)) = queue.pop() {
            if x == self.exit.0 && y == self.exit.1 {
                return steps;
            }
            if let Some(visited_steps) = visited.get(&(x, y)) {
                if visited_steps <= &steps {
                    continue;
                }
            }
            visited.insert((x, y), steps);
            let mut possible_moves = Vec::new();
            if Tile::Empty == self.tiles[y][x] {
                if 0 < y
                    && Tile::Wall != self.tiles[y - 1][x]
                    && Tile::SouthDownSlope != self.tiles[y - 1][x]
                {
                    possible_moves.push((x, y - 1));
                }
                if 0 < x
                    && Tile::Wall != self.tiles[y][x - 1]
                    && Tile::EastDownSlope != self.tiles[y][x - 1]
                {
                    possible_moves.push((x - 1, y));
                }
                if y < self.tiles.len() - 1
                    && Tile::Wall != self.tiles[y + 1][x]
                    && Tile::NorthDownSlope != self.tiles[y + 1][x]
                {
                    possible_moves.push((x, y + 1));
                }
                if x < self.tiles[y].len() - 1
                    && Tile::Wall != self.tiles[y][x + 1]
                    && Tile::WestDownSlope != self.tiles[y][x + 1]
                {
                    possible_moves.push((x + 1, y));
                }
            } else if Tile::NorthDownSlope == self.tiles[y][x] {
                possible_moves.push((x, y - 1));
            } else if Tile::EastDownSlope == self.tiles[y][x] {
                possible_moves.push((x + 1, y));
            } else if Tile::SouthDownSlope == self.tiles[y][x] {
                possible_moves.push((x, y + 1));
            } else if Tile::WestDownSlope == self.tiles[y][x] {
                possible_moves.push((x - 1, y));
            }
            possible_moves.into_iter().for_each(|(new_x, new_y)| {
                let new_steps = steps + 1;
                queue.push((new_steps, new_x, new_y));
            });
        }
        unreachable!()
    }
}

fn part1(input: String) -> usize {
    let map = Map::from_string(input);
    map.find_longest_path()
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tile_from_char() {
        assert_eq!(Tile::Empty, Tile::from_char('.'));
        assert_eq!(Tile::Wall, Tile::from_char('#'));
        assert_eq!(Tile::NorthDownSlope, Tile::from_char('^'));
        assert_eq!(Tile::EastDownSlope, Tile::from_char('>'));
        assert_eq!(Tile::SouthDownSlope, Tile::from_char('v'));
        assert_eq!(Tile::WestDownSlope, Tile::from_char('<'));
    }

    #[test]
    fn parses_map_from_string() {
        let input = "##.#
        #..#
        #v##
        #.##
        "
        .to_string();
        let expected = Map {
            tiles: vec![
                vec![Tile::Wall, Tile::Wall, Tile::Empty, Tile::Wall],
                vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Wall],
                vec![Tile::Wall, Tile::SouthDownSlope, Tile::Wall, Tile::Wall],
                vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Wall],
            ],
            entrance: (2, 0),
            exit: (1, 3),
        };
        assert_eq!(expected, Map::from_string(input));
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            94,
            part1(
                "#.#####################
                #.......#########...###
                #######.#########.#.###
                ###.....#.>.>.###.#.###
                ###v#####.#v#.###.#.###
                ###.>...#.#.#.....#...#
                ###v###.#.#.#########.#
                ###...#.#.#.......#...#
                #####.#.#.#######.#.###
                #.....#.#.#.......#...#
                #.#####.#.#.#########v#
                #.#...#...#...###...>.#
                #.#.#v#######v###.###v#
                #...#.>.#...>.>.#.###.#
                #####v#.#.###v#.#.###.#
                #.....#...#...#.#.#...#
                #.#########.###.#.#.###
                #...###...#...#...#.###
                ###.###.#.###v#####v###
                #...#...#.#.>.>.#.>.###
                #.###.###.#.###.#.#v###
                #.....###...###...#...#
                #####################.#
                "
                .to_string()
            )
        );
    }
}

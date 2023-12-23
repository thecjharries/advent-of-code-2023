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

use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
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

fn part1(input: String) -> usize {
    todo!()
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

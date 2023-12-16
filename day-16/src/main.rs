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
    East,
    South,
    West,
    North,
}

#[derive(Debug, PartialEq, Eq)]
enum CellContents {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Beam(Vec<Direction>),
}

impl CellContents {
    fn from_char(character: char) -> Self {
        match character {
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => Self::Empty,
        }
    }

    fn next_move(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Self::ForwardMirror => match direction {
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
                Direction::North => vec![Direction::West],
            },
            Self::BackwardMirror => match direction {
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
                Direction::North => vec![Direction::East],
            },
            Self::VerticalSplitter => match direction {
                Direction::East => vec![Direction::South, Direction::North],
                Direction::West => vec![Direction::South, Direction::North],
                _ => vec![direction],
            },
            Self::HorizontalSplitter => match direction {
                Direction::South => vec![Direction::East, Direction::West],
                Direction::North => vec![Direction::East, Direction::West],
                _ => vec![direction],
            },
            _ => vec![direction],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MapCell {
    contents: CellContents,
    energized: bool,
}

impl MapCell {
    fn new_from_char(character: char) -> Self {
        Self {
            contents: CellContents::from_char(character),
            energized: false,
        }
    }
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
    fn test_cell_contents_from_char() {
        assert_eq!(CellContents::from_char('/'), CellContents::ForwardMirror);
        assert_eq!(CellContents::from_char('\\'), CellContents::BackwardMirror);
        assert_eq!(CellContents::from_char('|'), CellContents::VerticalSplitter);
        assert_eq!(
            CellContents::from_char('-'),
            CellContents::HorizontalSplitter
        );
        assert_eq!(CellContents::from_char('.'), CellContents::Empty);
    }

    #[test]
    fn movement_from_a_cell_matches_directions() {
        let contents = CellContents::ForwardMirror;
        assert_eq!(vec![Direction::South], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::East], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::North], contents.next_move(Direction::West));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::North));
        let contents = CellContents::BackwardMirror;
        assert_eq!(vec![Direction::North], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::South], contents.next_move(Direction::West));
        assert_eq!(vec![Direction::East], contents.next_move(Direction::North));
        let contents = CellContents::VerticalSplitter;
        assert_eq!(
            vec![Direction::South, Direction::North],
            contents.next_move(Direction::East)
        );
        assert_eq!(
            vec![Direction::South, Direction::North],
            contents.next_move(Direction::West)
        );
        assert_eq!(vec![Direction::South], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::North], contents.next_move(Direction::North));
        let contents = CellContents::HorizontalSplitter;
        assert_eq!(
            vec![Direction::East, Direction::West],
            contents.next_move(Direction::South)
        );
        assert_eq!(
            vec![Direction::East, Direction::West],
            contents.next_move(Direction::North)
        );
        assert_eq!(vec![Direction::East], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::West));
        let contents = CellContents::Empty;
        assert_eq!(vec![Direction::East], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::South], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::West));
        assert_eq!(vec![Direction::North], contents.next_move(Direction::North));
        let contents = CellContents::Beam(vec![Direction::East]);
        assert_eq!(vec![Direction::East], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::South], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::West));
        assert_eq!(vec![Direction::North], contents.next_move(Direction::North));
    }

    #[test]
    fn test_map_cell_new_from_char() {
        assert_eq!(
            MapCell::new_from_char('/'),
            MapCell {
                contents: CellContents::ForwardMirror,
                energized: false
            }
        );
    }
}

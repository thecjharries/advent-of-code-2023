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

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for CellContents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::ForwardMirror => write!(f, "/"),
            Self::BackwardMirror => write!(f, "\\"),
            Self::VerticalSplitter => write!(f, "|"),
            Self::HorizontalSplitter => write!(f, "-"),
            Self::Beam(directions) => {
                if 1 < directions.len() {
                    write!(f, "{}", directions.len())
                } else {
                    match directions[0] {
                        Direction::East => write!(f, ">"),
                        Direction::South => write!(f, "v"),
                        Direction::West => write!(f, "<"),
                        Direction::North => write!(f, "^"),
                    }
                }
            }
        }
    }
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
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
                Direction::North => vec![Direction::East],
            },
            Self::BackwardMirror => match direction {
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
                Direction::North => vec![Direction::West],
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
    entered_from: Vec<Direction>,
}

impl std::fmt::Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl MapCell {
    fn new_from_char(character: char) -> Self {
        Self {
            contents: CellContents::from_char(character),
            energized: false,
            entered_from: Vec::new(),
        }
    }

    fn next_move(&mut self, direction: Direction) -> Vec<Direction> {
        if self.entered_from.contains(&direction) {
            return Vec::new();
        } else {
            self.entered_from.push(direction.clone());
        }
        self.energized = true;
        if let CellContents::Beam(directions) = &self.contents {
            let mut new_directions = directions.clone();
            if !new_directions.contains(&direction) {
                new_directions.push(direction.clone());
                self.contents = CellContents::Beam(new_directions);
            }
        }
        if let CellContents::Empty = &self.contents {
            self.contents = CellContents::Beam(vec![direction.clone()]);
        }
        self.contents.next_move(direction)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    cells: Vec<Vec<MapCell>>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let input = input.trim();
        let mut cells = Vec::new();
        for line in input.lines() {
            let line = line.trim();
            let mut row = Vec::new();
            for character in line.chars() {
                row.push(MapCell::new_from_char(character));
            }
            cells.push(row);
        }
        Self {
            width: cells[0].len(),
            height: cells.len(),
            cells,
        }
    }

    fn get_energized_count(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|cell| cell.energized)
            .count()
    }

    fn energize(&mut self, start_x: usize, start_y: usize, direction: Direction) {
        let mut beams: Vec<(usize, usize, Direction)> = Vec::new();
        beams.push((start_x, start_y, direction));
        while !beams.is_empty() {
            let (x, y, direction) = beams.pop().unwrap();
            let cell = &mut self.cells[y][x];
            let next_moves = cell.next_move(direction);
            for next_move in next_moves {
                match next_move {
                    Direction::East => {
                        if x + 1 < self.width {
                            beams.push((x + 1, y, next_move));
                        }
                    }
                    Direction::South => {
                        if y + 1 < self.height {
                            beams.push((x, y + 1, next_move));
                        }
                    }
                    Direction::West => {
                        if x > 0 {
                            beams.push((x - 1, y, next_move));
                        }
                    }
                    Direction::North => {
                        if y > 0 {
                            beams.push((x, y - 1, next_move));
                        }
                    }
                }
            }
        }
    }
}

fn part1(input: String) -> usize {
    let mut map = Map::new(&input);
    map.energize(0, 0, Direction::East);
    map.get_energized_count()
}

#[cfg(not(tarpaulin_include))]
fn part2(input: String) -> usize {
    let mut map = Map::new(&input);
    map.energize(0, 0, Direction::East);
    let mut max_energized = map.get_energized_count();
    for start_x in 0..map.width {
        let mut top_down = Map::new(&input);
        top_down.energize(start_x, 0, Direction::South);
        if top_down.get_energized_count() > max_energized {
            max_energized = top_down.get_energized_count();
        }
        let mut bottom_up = Map::new(&input);
        bottom_up.energize(start_x, map.height - 1, Direction::North);
        if bottom_up.get_energized_count() > max_energized {
            max_energized = bottom_up.get_energized_count();
        }
    }
    for start_y in 0..map.height {
        let mut left_right = Map::new(&input);
        left_right.energize(0, start_y, Direction::East);
        if left_right.get_energized_count() > max_energized {
            max_energized = left_right.get_energized_count();
        }
        let mut right_left = Map::new(&input);
        right_left.energize(map.width - 1, start_y, Direction::West);
        if right_left.get_energized_count() > max_energized {
            max_energized = right_left.get_energized_count();
        }
    }
    max_energized
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
    fn cellcontents_can_print() {
        assert_eq!(CellContents::Empty.to_string(), ".");
        assert_eq!(CellContents::ForwardMirror.to_string(), "/");
        assert_eq!(CellContents::BackwardMirror.to_string(), "\\");
        assert_eq!(CellContents::VerticalSplitter.to_string(), "|");
        assert_eq!(CellContents::HorizontalSplitter.to_string(), "-");
        assert_eq!(CellContents::Beam(vec![Direction::East]).to_string(), ">");
        assert_eq!(
            CellContents::Beam(vec![Direction::East, Direction::South]).to_string(),
            "2"
        );
    }

    #[test]
    fn movement_from_a_cell_matches_directions() {
        let contents = CellContents::ForwardMirror;
        assert_eq!(vec![Direction::North], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::South], contents.next_move(Direction::West));
        assert_eq!(vec![Direction::East], contents.next_move(Direction::North));
        let contents = CellContents::BackwardMirror;
        assert_eq!(vec![Direction::South], contents.next_move(Direction::East));
        assert_eq!(vec![Direction::East], contents.next_move(Direction::South));
        assert_eq!(vec![Direction::North], contents.next_move(Direction::West));
        assert_eq!(vec![Direction::West], contents.next_move(Direction::North));
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
                energized: false,
                entered_from: Vec::new(),
            }
        );
    }

    #[test]
    fn map_can_print() {
        let input = "/--\\\n|..|\n|..|\n\\--/\n";
        let map = Map::new(input);
        assert_eq!(map.to_string(), input);
    }

    #[test]
    fn map_cells_can_properly_move() {
        let mut map_cell = MapCell::new_from_char('/');
        assert_eq!(vec![Direction::North], map_cell.next_move(Direction::East));
        assert!(map_cell.energized);
        let mut map_cell = MapCell::new_from_char('\\');
        assert_eq!(vec![Direction::South], map_cell.next_move(Direction::East));
        assert!(map_cell.energized);
        let mut map_cell = MapCell::new_from_char('|');
        assert_eq!(
            vec![Direction::South, Direction::North],
            map_cell.next_move(Direction::East)
        );
        assert!(map_cell.energized);
        let mut map_cell = MapCell::new_from_char('-');
        assert_eq!(
            vec![Direction::East, Direction::West],
            map_cell.next_move(Direction::South)
        );
        assert!(map_cell.energized);
        let mut map_cell = MapCell::new_from_char('.');
        assert_eq!(vec![Direction::East], map_cell.next_move(Direction::East));
        assert_eq!(CellContents::Beam(vec![Direction::East]), map_cell.contents);
        assert!(map_cell.energized);
        let mut map_cell = MapCell {
            contents: CellContents::Beam(vec![Direction::East]),
            energized: true,
            entered_from: Vec::new(),
        };
        assert_eq!(vec![Direction::East], map_cell.next_move(Direction::East));
        assert_eq!(CellContents::Beam(vec![Direction::East]), map_cell.contents);
        assert_eq!(vec![Direction::South], map_cell.next_move(Direction::South));
        assert_eq!(
            CellContents::Beam(vec![Direction::East, Direction::South]),
            map_cell.contents
        );
    }

    #[test]
    fn test_map_new() {
        let input = r#"/--\
                       |..|
                       |..|
                       \--/"#;
        let map = Map::new(input);
        assert_eq!(4, map.width);
        assert_eq!(4, map.height);
        assert_eq!(
            vec![
                MapCell {
                    contents: CellContents::ForwardMirror,
                    energized: false,
                    entered_from: Vec::new(),
                },
                MapCell {
                    contents: CellContents::HorizontalSplitter,
                    energized: false,
                    entered_from: Vec::new(),
                },
                MapCell {
                    contents: CellContents::HorizontalSplitter,
                    energized: false,
                    entered_from: Vec::new(),
                },
                MapCell {
                    contents: CellContents::BackwardMirror,
                    energized: false,
                    entered_from: Vec::new(),
                }
            ],
            map.cells[0]
        );
    }

    #[test]
    fn test_get_energized_count() {
        let input = r#"/--\
                       |..|
                       |..|
                       \--/"#;
        let map = Map::new(input);
        assert_eq!(0, map.get_energized_count());
    }

    #[test]
    fn energize_creates_necessary_beams() {
        let input = "..\n..";
        let mut map = Map::new(input);
        map.energize(0, 0, Direction::East);
        assert_eq!(2, map.get_energized_count());
        let input = "..\\\n..-\n...";
        let mut map = Map::new(input);
        map.energize(0, 0, Direction::East);
        assert_eq!(6, map.get_energized_count());
    }

    #[test]
    fn solves_part1() {
        let input = ".|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|....
        ";
        assert_eq!(46, part1(input.to_string()));
    }

    #[test]
    fn solves_part2() {
        let input = ".|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|....
        ";
        assert_eq!(51, part2(input.to_string()));
    }
}

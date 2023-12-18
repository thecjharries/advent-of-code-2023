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

use geo::{Area, Polygon};

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(character: char) -> Self {
        match character {
            'U' | '3' => Self::Up,
            'R' | '0' => Self::Right,
            'D' | '1' => Self::Down,
            _ => Self::Left,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Direction, usize, String)> {
    let input = input.trim();
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut line = line.split_whitespace();
            let direction = Direction::from_char(line.next().unwrap().chars().next().unwrap());
            let steps = line.next().unwrap().parse::<usize>().unwrap();
            let color = line.next().unwrap().to_string();
            (direction, steps, color)
        })
        .collect()
}

fn part1(input: String) -> usize {
    let directions = parse_input(&input);
    let mut vertices = vec![(0.0, 0.0)];
    let mut current_vertex = (0.0, 0.0);
    let mut trench = 0;
    for (direction, steps, _) in directions {
        match direction {
            Direction::Up => current_vertex.1 += steps as f64,
            Direction::Right => current_vertex.0 += steps as f64,
            Direction::Down => current_vertex.1 -= steps as f64,
            Direction::Left => current_vertex.0 -= steps as f64,
        }
        vertices.push(current_vertex);
        trench += steps;
    }
    let polygon = Polygon::new(vertices.into(), vec![]);
    polygon.unsigned_area() as usize + trench / 2 + 1
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_directions() {
        assert_eq!(Direction::Up, Direction::from_char('U'));
        assert_eq!(Direction::Right, Direction::from_char('R'));
        assert_eq!(Direction::Down, Direction::from_char('D'));
        assert_eq!(Direction::Left, Direction::from_char('L'));
    }

    #[test]
    fn parses_input() {
        // R 6 (#70c710)
        // D 5 (#0dc571)
        // L 2 (#5713f0)
        assert_eq!(
            vec![
                (Direction::Right, 6, "(#70c710)".to_string()),
                (Direction::Down, 5, "(#0dc571)".to_string()),
                (Direction::Left, 2, "(#5713f0)".to_string()),
            ],
            parse_input(
                "R 6 (#70c710)
                D 5 (#0dc571)
                L 2 (#5713f0)
                "
            )
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            62,
            part1(
                "R 6 (#70c710)
                D 5 (#0dc571)
                L 2 (#5713f0)
                D 2 (#d2c081)
                R 2 (#59c680)
                D 2 (#411b91)
                L 5 (#8ceee2)
                U 2 (#caa173)
                L 1 (#1b58a2)
                U 2 (#caa171)
                R 2 (#7807d2)
                U 3 (#a77fa3)
                L 2 (#015232)
                U 2 (#7a21e3)
                "
                .to_string()
            )
        );
    }
}

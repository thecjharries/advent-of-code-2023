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

use std::collections::BTreeMap;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(character: char) -> Self {
        match character.to_ascii_uppercase() {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction character"),
        }
    }
}

fn parse_map(input: &str) -> BTreeMap<String, BTreeMap<Direction, String>> {
    let input = input.trim().replace(" ", "");
    let mut map = BTreeMap::new();
    for line in input.lines() {
        let mut split = line.split("=");
        let key = split.next().unwrap();
        let value = split
            .next()
            .unwrap()
            .trim_matches(|character| character == '(' || character == ')');
        let mut directions = BTreeMap::new();
        let mut values = value.split(',');
        directions.insert(Direction::Left, values.next().unwrap().to_string());
        directions.insert(Direction::Right, values.next().unwrap().to_string());
        map.insert(key.to_string(), directions);
    }
    map
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
    #[should_panic]
    fn direction_from_char_panics_on_invalid_character() {
        Direction::from_char('X');
    }

    #[test]
    fn direction_from_char_returns_left_on_l() {
        assert_eq!(Direction::from_char('L'), Direction::Left);
    }

    #[test]
    fn direction_from_char_returns_right_on_r() {
        assert_eq!(Direction::from_char('R'), Direction::Right);
    }

    #[test]
    fn parse_map_returns_expected_map() {
        let input = "AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let mut expected = BTreeMap::new();
        expected.insert(
            "AAA".to_string(),
            vec![
                (Direction::Left, "BBB".to_string()),
                (Direction::Right, "BBB".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        expected.insert(
            "BBB".to_string(),
            vec![
                (Direction::Left, "AAA".to_string()),
                (Direction::Right, "ZZZ".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        expected.insert(
            "ZZZ".to_string(),
            vec![
                (Direction::Left, "ZZZ".to_string()),
                (Direction::Right, "ZZZ".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(parse_map(&input), expected);
    }
}

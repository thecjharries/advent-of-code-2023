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

fn parse_directions(input: &str) -> Vec<Direction> {
    let input = input.trim();
    input
        .chars()
        .map(Direction::from_char)
        .collect::<Vec<Direction>>()
}

fn part1(input: String) -> usize {
    let input = input.trim();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let directions = parse_directions(parts[0]);
    let map = parse_map(parts[1]);
    let mut current = "AAA".to_string();
    let mut index = 0;
    let mut steps = 0;
    while "ZZZ" != current {
        let direction = &directions[index];
        let next = map.get(&current).unwrap().get(&direction).unwrap();
        current = next.to_string();
        index = (index + 1) % directions.len();
        steps += 1;
    }
    steps
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part2(input: String) -> usize {
    let input = input.trim();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let directions = parse_directions(parts[0]);
    let map = parse_map(parts[1]);
    let mut index = 0;
    let mut steps = 0;
    let mut paths = Vec::new();
    for (key, _) in map.iter() {
        if key.ends_with('A') {
            paths.push(key.to_string());
        }
    }
    let mut cycle_lengths = Vec::new();
    for path in paths {
        let mut current = path;
        while !current.ends_with('Z') {
            let direction = &directions[index];
            let next = map.get(&current).unwrap().get(&direction).unwrap();
            current = next.to_string();
            index = (index + 1) % directions.len();
            steps += 1;
        }
        cycle_lengths.push(steps);
        steps = 0;
    }
    cycle_lengths.iter().fold(1, |acc, x| lcm(acc, *x))
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

    #[test]
    fn parse_directions_returns_expected_directions() {
        let input = "LRL".to_string();
        let expected = vec![Direction::Left, Direction::Right, Direction::Left];
        assert_eq!(parse_directions(&input), expected);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            2,
            part1(
                "RL

                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)
                "
                .to_string()
            )
        );
        assert_eq!(
            6,
            part1(
                "LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)
                "
                .to_string()
            )
        );
    }

    #[test]
    fn gcd_returns_equality() {
        assert_eq!(gcd(1, 1), 1);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            6,
            part2(
                "LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)
                "
                .to_string()
            )
        );
    }
}

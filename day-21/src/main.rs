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

use std::collections::HashSet;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn parse_map(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let input = input.trim();
    let mut map = Vec::new();
    let mut santa = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut row = Vec::new();
        for (x, character) in line.chars().enumerate() {
            if 'S' == character {
                santa = (x, y);
                row.push('.');
            } else {
                row.push(character);
            }
        }
        map.push(row);
    }
    (map, santa)
}

fn get_possible_tails_count(map: Vec<Vec<char>>, santa: (usize, usize), max_steps: usize) -> usize {
    let mut traversed = HashSet::new();
    let mut available = HashSet::new();
    available.insert(santa);
    for _ in 0..max_steps {
        traversed = available.clone();
        available.clear();
        for (x, y) in traversed.iter() {
            if 0 < *x && '.' == map[*y][*x - 1] {
                available.insert((*x - 1, *y));
            }
            if 0 < *y && '.' == map[*y - 1][*x] {
                available.insert((*x, *y - 1));
            }
            if map[*y].len() > *x + 1 && '.' == map[*y][*x + 1] {
                available.insert((*x + 1, *y));
            }
            if map.len() > *y + 1 && '.' == map[*y + 1][*x] {
                available.insert((*x, *y + 1));
            }
        }
    }
    available.len()
}

fn part1(input: String) -> usize {
    let (map, santa) = parse_map(&input);
    get_possible_tails_count(map, santa, 6)
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_map() {
        let input = "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
        ";
        let (map, santa) = parse_map(input);
        assert_eq!(11, map.len());
        assert_eq!(11, map[0].len());
        assert_eq!((5, 5), santa);
    }

    #[test]
    fn counts_proper_number_of_tails() {
        let (map, santa) = parse_map(
            "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
        ",
        );
        assert_eq!(2, get_possible_tails_count(map.clone(), santa, 1));
        assert_eq!(4, get_possible_tails_count(map.clone(), santa, 2));
        assert_eq!(16, get_possible_tails_count(map, santa, 6));
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Legend {
    Space,
    Galaxy,
}

impl Legend {
    fn from_char(character: char) -> Option<Self> {
        match character {
            '.' => Some(Legend::Space),
            '#' => Some(Legend::Galaxy),
            _ => None,
        }
    }
}

fn parse_map(input: &str) -> (Vec<Vec<Legend>>, HashSet<(usize, usize)>) {
    let input = input.trim();
    let mut map = Vec::new();
    let mut galaxies = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut row = Vec::new();
        for (x, character) in line.chars().enumerate() {
            if let Some(legend) = Legend::from_char(character) {
                row.push(legend);
                if Legend::Galaxy == legend {
                    galaxies.insert((x, y));
                }
            }
        }
        map.push(row);
    }
    (map, galaxies)
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
    fn legend_parses_from_chars() {
        assert_eq!(Some(Legend::Space), Legend::from_char('.'));
        assert_eq!(Some(Legend::Galaxy), Legend::from_char('#'));
        assert_eq!(None, Legend::from_char('!'));
    }

    #[test]
    fn map_parses_from_input() {
        let output_map = vec![
            vec![Legend::Galaxy, Legend::Space, Legend::Space],
            vec![Legend::Space, Legend::Space, Legend::Space],
            vec![Legend::Space, Legend::Space, Legend::Galaxy],
        ];
        let output_galaxies: HashSet<(usize, usize)> = vec![(0, 0), (2, 2)].into_iter().collect();
        assert_eq!(
            (output_map, output_galaxies),
            parse_map(
                "#..
        ...
        ..#
        "
            )
        );
    }
}

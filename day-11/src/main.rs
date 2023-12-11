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

fn parse_map(input: &str) -> Vec<Vec<Legend>> {
    let input = input.trim();
    let mut map = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut row = Vec::new();
        for (x, character) in line.chars().enumerate() {
            if let Some(legend) = Legend::from_char(character) {
                row.push(legend);
            }
        }
        map.push(row);
    }
    map
}

fn expand_galaxy(galaxy: Vec<Vec<Legend>>) -> Vec<Vec<Legend>> {
    let mut expanded = Vec::new();
    for row in galaxy.clone() {
        let count = row.clone().iter().fold(0, |acc, legend| match legend {
            Legend::Galaxy => acc + 1,
            _ => acc,
        });
        if 0 == count {
            expanded.push(row.clone());
        }
        expanded.push(row);
    }
    let mut column = 0;
    while column < galaxy[0].len() {
        let count = galaxy.clone().iter().fold(0, |acc, row| match row[column] {
            Legend::Galaxy => acc + 1,
            _ => acc,
        });
        if 0 == count {
            for row in &mut expanded {
                row.insert(column, Legend::Space);
            }
        }
        column += 1;
    }
    expanded
}

fn find_galaxies(galaxy: Vec<Vec<Legend>>) -> HashSet<(usize, usize)> {
    let mut galaxies = HashSet::new();
    for (y, row) in galaxy.iter().enumerate() {
        for (x, legend) in row.iter().enumerate() {
            if Legend::Galaxy == *legend {
                galaxies.insert((x, y));
            }
        }
    }
    galaxies
}

fn find_shortest_manhattan_distance(first: (usize, usize), second: (usize, usize)) -> usize {
    let x_distance = if first.0 > second.0 {
        first.0 - second.0
    } else {
        second.0 - first.0
    };
    let y_distance = if first.1 > second.1 {
        first.1 - second.1
    } else {
        second.1 - first.1
    };
    x_distance + y_distance
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
        assert_eq!(
            output_map,
            parse_map(
                "#..
        ...
        ..#
        "
            )
        );
    }

    #[test]
    fn galaxy_expands_empty_rows_and_columns() {
        let output_map = vec![
            vec![Legend::Galaxy, Legend::Space, Legend::Space, Legend::Space],
            vec![Legend::Space, Legend::Space, Legend::Space, Legend::Space],
            vec![Legend::Space, Legend::Space, Legend::Space, Legend::Space],
            vec![Legend::Space, Legend::Space, Legend::Space, Legend::Galaxy],
        ];
        assert_eq!(
            output_map,
            expand_galaxy(vec![
                vec![Legend::Galaxy, Legend::Space, Legend::Space],
                vec![Legend::Space, Legend::Space, Legend::Space],
                vec![Legend::Space, Legend::Space, Legend::Galaxy],
            ])
        );
    }

    #[test]
    fn can_find_all_galaxies() {
        let galaxies: HashSet<(usize, usize)> = vec![(0, 0), (2, 2)].into_iter().collect();
        assert_eq!(
            galaxies,
            find_galaxies(vec![
                vec![Legend::Galaxy, Legend::Space, Legend::Space],
                vec![Legend::Space, Legend::Space, Legend::Space],
                vec![Legend::Space, Legend::Space, Legend::Galaxy],
            ])
        );
    }

    #[test]
    fn shortest_manhattan_distance_is_xy_sum() {
        assert_eq!(15, find_shortest_manhattan_distance((0, 4), (10, 9)));
        assert_eq!(17, find_shortest_manhattan_distance((2, 0), (7, 12)));
    }
}

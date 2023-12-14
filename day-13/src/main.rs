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

fn transpose(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for column in 0..input[0].len() {
        let mut row = Vec::new();
        for row_index in 0..input.len() {
            row.push(input[row_index][column]);
        }
        output.push(row);
    }
    output
}

fn compare_sides(first: Vec<Vec<char>>, second: Vec<Vec<char>>) -> bool {
    let mut first = first;
    let mut second = second;
    if first.len() > second.len() {
        first = first[first.len() - second.len()..].to_vec();
    }
    if second.len() > first.len() {
        second = second[0..first.len()].to_vec();
    }
    second.reverse();
    first == second
}

fn find_reflection(input: Vec<Vec<char>>) -> usize {
    for index in 1..input.len() {
        if compare_sides(input[0..index].to_vec(), input[index..].to_vec()) {
            return 100 * index;
        }
    }
    let input = transpose(input);
    for index in 1..input.len() {
        if compare_sides(input[0..index].to_vec(), input[index..].to_vec()) {
            return index;
        }
    }
    0
}

fn parse_input_to_maps(input: &str) -> Vec<Vec<Vec<char>>> {
    let input = input.trim();
    input
        .split("\n\n")
        .map(|map| {
            let map = map.trim();
            map.split("\n")
                .map(|line| {
                    let line = line.trim();
                    line.chars().collect()
                })
                .collect()
        })
        .collect()
}

fn part1(input: String) -> usize {
    let maps = parse_input_to_maps(&input);
    maps.iter().map(|map| find_reflection(map.clone())).sum()
}

#[cfg(not(tarpaulin_include))]
fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_works_as_expected() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
            vec!['*', '0', '#'],
        ];
        let expected = vec![
            vec!['1', '4', '7', '*'],
            vec!['2', '5', '8', '0'],
            vec!['3', '6', '9', '#'],
        ];
        assert_eq!(expected, transpose(input));
    }

    #[test]
    fn compare_sides_handles_different_sizes() {
        assert!(compare_sides(
            vec![
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            ],
            vec![
                vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
            ]
        ));
    }

    #[test]
    fn can_find_reflection_lines_above() {
        let input = vec![
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
        ];
        assert_eq!(400, find_reflection(input));
        let input = vec![
            vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
            vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
            vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'],
        ];
        assert_eq!(5, find_reflection(input));
    }

    #[test]
    fn parses_input_to_maps() {
        let input = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        ";
        let expected = vec![
            // #.##..##.
            // ..#.##.#.
            // ##......#
            // ##......#
            // ..#.##.#.
            // ..##..##.
            // #.#.##.#.
            vec![
                vec!['#', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['#', '#', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '#', '.', '#', '#', '.', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '.'],
                vec!['#', '.', '#', '.', '#', '#', '.', '#', '.'],
            ],
            // #...##..#
            // #....#..#
            // ..##..###
            // #####.##.
            // #####.##.
            // ..##..###
            // #....#..#
            vec![
                vec!['#', '.', '.', '.', '#', '#', '.', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
                vec!['#', '#', '#', '#', '#', '.', '#', '#', '.'],
                vec!['.', '.', '#', '#', '.', '.', '#', '#', '#'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '#'],
            ],
        ];
        assert_eq!(expected, parse_input_to_maps(input));
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            405,
            part1(
                "#.##..##.
                ..#.##.#.
                ##......#
                ##......#
                ..#.##.#.
                ..##..##.
                #.#.##.#.

                #...##..#
                #....#..#
                ..##..###
                #####.##.
                #####.##.
                ..##..###
                #....#..#
                "
                .to_string()
            )
        )
    }
}

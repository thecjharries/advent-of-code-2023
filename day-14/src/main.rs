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

fn rotate_matrix_counter_clockwise(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for column in 0..input[0].len() {
        let mut row = Vec::new();
        for row_index in 0..input.len() {
            row.push(input[row_index][column]);
        }
        output.push(row);
    }
    output.reverse();
    output
}

fn tilt_transposed_north(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for row in input {
        let mut first_free_space = 0;
        let mut output_row = row.clone();
        for (index, character) in row.iter().enumerate() {
            if 'O' == *character {
                output_row.swap(first_free_space, index);
                first_free_space += 1;
            }
            if '#' == *character {
                first_free_space = index + 1;
            }
        }
        output.push(output_row);
    }
    output
}

fn parse_input_to_map(input: &str) -> Vec<Vec<char>> {
    let input = input.trim();
    input
        .split("\n")
        .map(|line| {
            let line = line.trim();
            line.chars().collect()
        })
        .collect()
}

fn part1(input: String) -> usize {
    let map = transpose(parse_input_to_map(&input));
    let mut map = tilt_transposed_north(map);
    map.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(index, character)| {
                    if 'O' == *character {
                        row.len() - index
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

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
    fn rotate_matrix_counter_clockwise_works_as_expected() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
            vec!['*', '0', '#'],
        ];
        let expected = vec![
            vec!['3', '6', '9', '#'],
            vec!['2', '5', '8', '0'],
            vec!['1', '4', '7', '*'],
        ];
        let output = rotate_matrix_counter_clockwise(input.clone());
        assert_eq!(expected, output);
        let expected = vec![
            vec!['#', '0', '*'],
            vec!['9', '8', '7'],
            vec!['6', '5', '4'],
            vec!['3', '2', '1'],
        ];
        let output = rotate_matrix_counter_clockwise(output);
        assert_eq!(expected, output);
        let expected = vec![
            vec!['*', '7', '4', '1'],
            vec!['0', '8', '5', '2'],
            vec!['#', '9', '6', '3'],
        ];
        let output = rotate_matrix_counter_clockwise(output);
        assert_eq!(expected, output);
        let output = rotate_matrix_counter_clockwise(output);
        assert_eq!(input, output);
    }

    #[test]
    fn can_parse_input_maps() {
        let input = "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        ";
        let expected = vec![
            // O....#....
            vec!['O', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            // O.OO#....#
            vec!['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#'],
            // .....##...
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            // OO.#O....O
            vec!['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O'],
            // .O.....O#.
            vec!['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.'],
            // O.#..O.#.#
            vec!['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#'],
            // ..O..#O..O
            vec!['.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O'],
            // .......O..
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.'],
            // #....###..
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            // #OO..#....
            vec!['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.'],
        ];
        assert_eq!(expected, parse_input_to_map(input));
    }

    #[test]
    fn can_tilt_transposed_maps() {
        let input = transpose(parse_input_to_map(
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....",
        ));
        let expected = transpose(parse_input_to_map(
            "OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#....",
        ));
        assert_eq!(expected, tilt_transposed_north(input));
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            136,
            part1(
                "O....#....
                O.OO#....#
                .....##...
                OO.#O....O
                .O.....O#.
                O.#..O.#.#
                ..O..#O..O
                .......O..
                #....###..
                #OO..#....
                "
                .to_string()
            )
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            64,
            part2(
                "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
            "
                .to_string()
            )
        );
    }
}

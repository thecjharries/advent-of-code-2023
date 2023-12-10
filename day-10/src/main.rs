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

fn parse_grid(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let input = input.trim();
    let mut row_index = 0;
    let mut start = (0, 0);
    let grid = input
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut row = Vec::new();
            for (index, character) in line.chars().enumerate() {
                if character == 'S' {
                    start = (row_index, index);
                }
                row.push(character);
            }
            row_index += 1;
            row
        })
        .collect::<Vec<Vec<char>>>();
    (grid, start)
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
    fn can_parse_input_into_grid() {
        // -L|F7
        // 7S-7|
        // L|7||
        // -L-J|
        // L|-JF
        let output = vec![
            vec!['-', 'L', '|', 'F', '7'],
            vec!['7', 'S', '-', '7', '|'],
            vec!['L', '|', '7', '|', '|'],
            vec!['-', 'L', '-', 'J', '|'],
            vec!['L', '|', '-', 'J', 'F'],
        ];
        assert_eq!(
            (output, (1, 1)),
            parse_grid(
                "-L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
            "
            )
        );
    }
}

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

#[derive(Debug, PartialEq, Eq)]
enum SymbolType {
    VerticalPipe,
    HorizontalPipe,
    EllBend,
    JayBend,
    SevenBend,
    FBend,
    Ground,
    Start,
}

impl SymbolType {
    fn from_char(character: char) -> Option<Self> {
        match character {
            '|' => Some(Self::VerticalPipe),
            '-' => Some(Self::HorizontalPipe),
            'L' => Some(Self::EllBend),
            'J' => Some(Self::JayBend),
            '7' => Some(Self::SevenBend),
            'F' => Some(Self::FBend),
            'S' => Some(Self::Start),
            '.' => Some(Self::Ground),
            _ => None,
        }
    }
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

    #[test]
    fn can_parse_symbols() {
        assert_eq!(Some(SymbolType::VerticalPipe), SymbolType::from_char('|'));
        assert_eq!(Some(SymbolType::HorizontalPipe), SymbolType::from_char('-'));
        assert_eq!(Some(SymbolType::EllBend), SymbolType::from_char('L'));
        assert_eq!(Some(SymbolType::JayBend), SymbolType::from_char('J'));
        assert_eq!(Some(SymbolType::SevenBend), SymbolType::from_char('7'));
        assert_eq!(Some(SymbolType::FBend), SymbolType::from_char('F'));
        assert_eq!(Some(SymbolType::Ground), SymbolType::from_char('.'));
        assert_eq!(Some(SymbolType::Start), SymbolType::from_char('S'));
        assert_eq!(None, SymbolType::from_char('x'));
    }
}

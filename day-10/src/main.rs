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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SymbolType {
    VerticalPipe,
    HorizontalPipe,
    EllBend,
    JayBend,
    SevenBend,
    EffBend,
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
            'F' => Some(Self::EffBend),
            'S' => Some(Self::Start),
            '.' => Some(Self::Ground),
            _ => None,
        }
    }

    fn get_neighbors(&self) -> Vec<(i32, i32)> {
        match self {
            Self::VerticalPipe => vec![(-1, 0), (1, 0)],
            Self::HorizontalPipe => vec![(0, -1), (0, 1)],
            Self::EllBend => vec![(-1, 0), (0, 1)],
            Self::JayBend => vec![(0, -1), (-1, 0)],
            Self::SevenBend => vec![(0, -1), (1, 0)],
            Self::EffBend => vec![(0, 1), (1, 0)],
            Self::Ground => vec![],
            Self::Start => vec![],
        }
    }
}

fn find_longest_steps(
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    inferred_symbol: SymbolType,
) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut neighbors = inferred_symbol.get_neighbors();
    let mut first_path = (
        (start.0 as i32 + neighbors[0].0) as usize,
        (start.1 as i32 + neighbors[0].1) as usize,
    );
    let mut second_path = (
        (start.0 as i32 + neighbors[1].0) as usize,
        (start.1 as i32 + neighbors[1].1) as usize,
    );
    let mut steps = 0;
    while !visited.contains(&first_path) && !visited.contains(&second_path) {
        visited.insert(first_path);
        visited.insert(second_path);
        let first_symbol = SymbolType::from_char(grid[first_path.0][first_path.1]).unwrap();
        let second_symbol = SymbolType::from_char(grid[second_path.0][second_path.1]).unwrap();
        neighbors = first_symbol.get_neighbors();
        for neighbor in neighbors {
            let neighbor = (
                (first_path.0 as i32 + neighbor.0) as usize,
                (first_path.1 as i32 + neighbor.1) as usize,
            );
            if !visited.contains(&neighbor) {
                first_path = neighbor;
                break;
            }
        }
        neighbors = second_symbol.get_neighbors();
        for neighbor in neighbors {
            let neighbor = (
                (second_path.0 as i32 + neighbor.0) as usize,
                (second_path.1 as i32 + neighbor.1) as usize,
            );
            if !visited.contains(&neighbor) {
                second_path = neighbor;
                break;
            }
        }
        steps += 1;
    }
    steps
}

fn part1(input: String) -> usize {
    let (grid, start) = parse_grid(&input);
    find_longest_steps(grid, start, SymbolType::HorizontalPipe)
}

fn part2(input: String) -> usize {
    let (grid, start) = parse_grid(&input);
    let mut grid = grid.clone();
    grid[start.0][start.1] = '-';
    let mut in_loop = HashSet::new();
    let mut path = vec![start];
    while !path.is_empty() {
        let current = path.pop().unwrap();
        let current_symbol = SymbolType::from_char(grid[current.0][current.1]).unwrap();
        let neighbors = current_symbol.get_neighbors();
        for neighbor in neighbors {
            let neighbor = (
                (current.0 as i32 + neighbor.0) as usize,
                (current.1 as i32 + neighbor.1) as usize,
            );
            if !in_loop.contains(&neighbor) {
                in_loop.insert(neighbor);
                path.push(neighbor);
            }
        }
    }
    let mut inside_count = 0;
    for row_index in 0..grid.len() {
        for column_index in 0..grid[row_index].len() {
            if in_loop.contains(&(row_index, column_index)) {
                continue;
            }
            let mut hit_loop = 0;
            for index in 0..=row_index {
                let current = (row_index - index, column_index);
                if in_loop.contains(&current) {
                    hit_loop += 1;
                }
            }
            if 0 == hit_loop % 2 {
                continue;
            }
            let mut hit_loop = 0;
            for index in 0..=column_index {
                let current = (row_index, column_index - index);
                if in_loop.contains(&current) {
                    hit_loop += 1;
                }
            }
            if 0 == hit_loop % 2 {
                continue;
            }
            let mut hit_loop = 0;
            for index in 0..=(grid.len() - row_index) {
                let current = (row_index + index, column_index);
                if in_loop.contains(&current) {
                    hit_loop += 1;
                }
            }
            if 0 == hit_loop % 2 {
                continue;
            }
            let mut hit_loop = 0;
            for index in 0..=(grid[row_index].len() - column_index) {
                let current = (row_index, column_index + index);
                if in_loop.contains(&current) {
                    hit_loop += 1;
                }
            }
            if 0 == hit_loop % 2 {
                continue;
            }
            println!("{} {}", row_index, column_index);
            inside_count += 1;
        }
    }
    inside_count
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
        assert_eq!(Some(SymbolType::EffBend), SymbolType::from_char('F'));
        assert_eq!(Some(SymbolType::Ground), SymbolType::from_char('.'));
        assert_eq!(Some(SymbolType::Start), SymbolType::from_char('S'));
        assert_eq!(None, SymbolType::from_char('x'));
    }

    #[test]
    fn can_get_neighbors() {
        assert_eq!(
            vec![(-1, 0), (1, 0)],
            SymbolType::VerticalPipe.get_neighbors()
        );
        assert_eq!(
            vec![(0, -1), (0, 1)],
            SymbolType::HorizontalPipe.get_neighbors()
        );
        assert_eq!(vec![(-1, 0), (0, 1)], SymbolType::EllBend.get_neighbors());
        assert_eq!(vec![(0, -1), (-1, 0)], SymbolType::JayBend.get_neighbors());
        assert_eq!(vec![(0, -1), (1, 0)], SymbolType::SevenBend.get_neighbors());
        assert_eq!(vec![(0, 1), (1, 0)], SymbolType::EffBend.get_neighbors());
        assert_eq!(
            vec![] as Vec<(i32, i32)>,
            SymbolType::Ground.get_neighbors()
        );
        assert_eq!(vec![] as Vec<(i32, i32)>, SymbolType::Start.get_neighbors());
    }

    #[test]
    fn can_find_longest_steps() {
        let (grid, start) = parse_grid(
            "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
        ",
        );
        assert_eq!(4, find_longest_steps(grid, start, SymbolType::EffBend));
        let (grid, start) = parse_grid(
            "7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
            ",
        );
        assert_eq!((2, 0), start);
        assert_eq!(8, find_longest_steps(grid, start, SymbolType::EffBend));
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            4,
            part2(
                "...........
                .S-------7.
                .|F-----7|.
                .||OOOOO||.
                .||OOOOO||.
                .|L-7OF-J|.
                .|II|O|II|.
                .L--JOL--J.
                .....O.....
                "
                .to_string()
            )
        );
        assert_eq!(
            4,
            part2(
                "..........
                .S------7.
                .|F----7|.
                .||OOOO||.
                .||OOOO||.
                .|L-7F-J|.
                .|II||II|.
                .L--JL--J.
                ..........
                "
                .to_string()
            )
        );
        assert_eq!(
            8,
            part2(
                ".F----7F7F7F7F-7....
                .|F--7||||||||FJ....
                .||.FJ||||||||L7....
                FJL7L7LJLJ||LJ.L-7..
                L--J.L7...LJS7F-7L7.
                ....F-J..F7FJ|L7L7L7
                ....L7.F7||L7|.L7L7|
                .....|FJLJ|FJ|F7|.LJ
                ....FJL-7.||.||||...
                ....L---J.LJ.LJLJ...
                "
                .to_string()
            )
        );
        assert_eq!(
            10,
            part2(
                "FF7FSF7F7F7F7F7F---7
                L|LJ||||||||||||F--J
                FL-7LJLJ||||||LJL-77
                F--JF--7||LJLJ7F7FJ-
                L---JF-JLJ.||-FJLJJ7
                |F|F-JF---7F7-L7L|7|
                |FFJF7L7F-JF7|JL---7
                7-L-JL7||F7|L7F-7F7|
                L.L7LFJ|||||FJL7||LJ
                L7JLJL-JLJLJL--JLJ.L
                "
                .to_string()
            )
        );
    }
}

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

fn part1(input: String) -> u64 {
    let input = input.trim();
    let mut grid = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut numbers: Vec<u64> = Vec::new();
    for row in 0..grid.len() {
        let mut is_number = false;
        let mut current_number = String::new();
        let mut is_part_number = false;
        for column in 0..grid[row].len() {
            if grid[row][column].is_digit(10) {
                is_number = true;
                current_number.push(grid[row][column]);
                if is_part_number {
                    continue;
                }
                for row_offset in -1..=1 {
                    for column_offset in -1..=1 {
                        if row_offset == 0 && column_offset == 0 {
                            continue;
                        }
                        let neighbor_row = row as i64 + row_offset;
                        let neighbor_column = column as i64 + column_offset;
                        if neighbor_row < 0
                            || neighbor_row >= grid.len() as i64
                            || neighbor_column < 0
                            || neighbor_column >= grid[row].len() as i64
                            || grid[neighbor_row as usize][neighbor_column as usize].is_digit(10)
                            || grid[neighbor_row as usize][neighbor_column as usize] == '.'
                        {
                            continue;
                        }
                        is_part_number = true;
                    }
                }
            } else if is_number {
                is_number = false;
                if is_part_number {
                    numbers.push(current_number.parse::<u64>().unwrap());
                }
                current_number = String::new();
                is_part_number = false;
            }
        }
    }
    numbers.iter().sum()
}

fn part2(input: String) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1() {
        assert_eq!(
            4361,
            part1(
                "467..114..
                ...*......
                ..35..633.
                ......#...
                617*......
                .....+.58.
                ..592.....
                ......755.
                ...$.*....
                .664.598..
                "
                .to_string()
            )
        );
    }
}

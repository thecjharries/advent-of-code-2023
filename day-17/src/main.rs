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

fn parse_map(input: &str) -> Vec<Vec<usize>> {
    let input = input.trim();
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            line.chars()
                .map(|character| character.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
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
    fn can_properly_parse_map() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],],
            parse_map(
                "123
                456
                789
                "
            )
        );
    }

    #[test]
    #[should_panic]
    fn panics_when_parsing_map_with_non_digit_characters() {
        parse_map(
            "123
            4a6
            789
            ",
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            102,
            part1(
                "2413432311323
                3215453535623
                3255245654254
                3446585845452
                4546657867536
                1438598798454
                4457876987766
                3637877979653
                4654967986887
                4564679986453
                1224686865563
                2546548887735
                4322674655533
                "
                .to_string()
            )
        );
    }
}

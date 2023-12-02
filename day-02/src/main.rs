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

struct Part1Game {
    id: u32,
    max_green: u32,
    max_red: u32,
    max_blue: u32,
}

fn parse_part1_game(input: &str) -> Part1Game {
    let mut parts = input.split(": ");
    let id = parts
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut parts = parts.next().unwrap().split("; ");
    let mut max_green = 0;
    let mut max_red = 0;
    let mut max_blue = 0;
    for part in parts.next().unwrap().split(", ") {
        let mut part = part.split(" ");
        let number = part.next().unwrap().parse::<u32>().unwrap();
        let color = part.next().unwrap();
        match color {
            "green" => max_green = number,
            "red" => max_red = number,
            "blue" => max_blue = number,
            _ => panic!("Unknown color {}", color),
        }
    }
    Part1Game {
        id,
        max_green,
        max_red,
        max_blue,
    }
}

fn part1(input: String) {
    todo!()
}

fn part2(input: String) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_part1_game() {
        assert_eq!(
            Part1Game {
                id: 3,
                max_green: 13,
                max_red: 20,
                max_blue: 6,
            },
            parse_part1_game(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            )
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            8,
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "
                .to_string()
            )
        );
    }
}

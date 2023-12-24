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

#[derive(Debug, PartialEq, Eq)]
struct Hailstone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Hailstone {
    fn from_str(input: &str) -> Self {
        let input = input.trim();
        let mut parts = input.split(" @");
        let position = parts
            .next()
            .expect("Unable to get position")
            .split(',')
            .map(|x| x.trim().parse::<isize>().expect("Unable to parse position"))
            .collect::<Vec<isize>>();
        let velocity = parts
            .next()
            .expect("Unable to get velocity")
            .split(',')
            .map(|x| x.trim().parse::<isize>().expect("Unable to parse velocity"))
            .collect::<Vec<isize>>();
        Self {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
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
    fn hailstone_parses_from_input() {
        assert_eq!(
            Hailstone {
                position: (19, 13, 30),
                velocity: (-2, 1, -2)
            },
            Hailstone::from_str("19, 13, 30 @ -2,  1, -2")
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            2,
            part1(
                "19, 13, 30 @ -2,  1, -2
                18, 19, 22 @ -1, -1, -2
                20, 25, 34 @ -2, -2, -4
                12, 31, 28 @ -1, -2, -1
                20, 19, 15 @  1, -5, -3
                "
                .to_string()
            )
        )
    }
}

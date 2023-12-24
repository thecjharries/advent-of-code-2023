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

use itertools::Itertools;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Hailstone {
    fn from_str(input: &str) -> Self {
        let input = input.trim();
        let mut parts = input.split(" @");
        let position = parts
            .next()
            .expect("Unable to get position")
            .split(',')
            .map(|x| x.trim().parse::<f64>().expect("Unable to parse position"))
            .collect::<Vec<f64>>();
        let velocity = parts
            .next()
            .expect("Unable to get velocity")
            .split(',')
            .map(|x| x.trim().parse::<f64>().expect("Unable to parse velocity"))
            .collect::<Vec<f64>>();
        Self {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
        }
    }

    fn crosses_pathes_in_test_area(&self, other: &Self, min: f64, max: f64) -> bool {
        let determinant =
            (self.velocity.1 * other.velocity.0) - (self.velocity.0 * other.velocity.1);
        if 0.0 == determinant {
            return false;
        }
        let d_x = other.position.0 - self.position.0;
        let d_y = other.position.1 - self.position.1;
        let u = ((self.velocity.0 * d_y) - (self.velocity.1 * d_x)) / determinant;
        if u < 0.0 {
            return false;
        }
        let v = ((other.velocity.0 * d_y) - (other.velocity.1 * d_x)) / determinant;
        if v < 0.0 {
            return false;
        }
        let point = (
            (self.position.0 + (v * self.velocity.0)),
            (self.position.1 + (u * self.velocity.1)),
        );
        if point.0 < min || point.0 > max || point.1 < min || point.1 > max {
            return false;
        }
        true
    }
}

fn parse_input(input: String) -> Vec<Hailstone> {
    let input = input.trim();
    input
        .lines()
        .map(|x| Hailstone::from_str(x))
        .collect::<Vec<Hailstone>>()
}

fn find_intersection_count_in_test_area(hailstones: Vec<Hailstone>, min: f64, max: f64) -> usize {
    let mut intersection_count = 0;
    for combination in hailstones.iter().combinations(2) {
        if combination[0].crosses_pathes_in_test_area(combination[1], min, max) {
            intersection_count += 1;
        }
    }
    intersection_count
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
                position: (19.0, 13.0, 30.0),
                velocity: (-2.0, 1.0, -2.0)
            },
            Hailstone::from_str("19, 13, 30 @ -2,  1, -2")
        );
    }

    #[test]
    fn hailstone_crosses_pathes_in_test_area() {
        let hailstone = Hailstone {
            position: (19.0, 13.0, 30.0),
            velocity: (-2.0, 1.0, -2.0),
        };
        assert!(hailstone.crosses_pathes_in_test_area(
            &Hailstone {
                position: (18.0, 19.0, 22.0),
                velocity: (-1.0, -1.0, -2.0)
            },
            7.0,
            27.0
        ));
    }

    #[test]
    fn parses_all_hailstones_from_input() {
        assert_eq!(
            vec![Hailstone {
                position: (19.0, 13.0, 30.0),
                velocity: (-2.0, 1.0, -2.0)
            }],
            parse_input(
                "
                19, 13, 30 @ -2,  1, -2
                "
                .to_string()
            )
        );
    }

    #[test]
    fn finds_all_intersections_in_test_area() {
        let hailstones = parse_input(
            "19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
            "
            .to_string(),
        );
        assert_eq!(
            2,
            find_intersection_count_in_test_area(hailstones, 7.0, 27.0)
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

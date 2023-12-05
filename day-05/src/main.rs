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

use std::collections::BTreeMap;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn parse_to_map(input: &str) -> BTreeMap<usize, usize> {
    let input = input.trim();
    let mut result = BTreeMap::new();
    for line in input.lines() {
        let line = line.trim();
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let value = parts[0].parse::<usize>().unwrap();
        let key = parts[1].parse::<usize>().unwrap();
        let max = parts[2].parse::<usize>().unwrap();

        for index in 0..max {
            result.insert(key + index, value + index);
        }
    }
    result
}

fn parse_seeds(input: &str) -> Vec<usize> {
    let input = input.trim().strip_prefix("seeds: ").unwrap();
    input
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
struct Garden {
    seeds: Vec<usize>,
    soil_to_fertilizer: BTreeMap<usize, usize>,
    fertilizer_to_water: BTreeMap<usize, usize>,
    water_to_light: BTreeMap<usize, usize>,
    light_to_temperature: BTreeMap<usize, usize>,
    temperature_to_humidity: BTreeMap<usize, usize>,
    humidity_to_location: BTreeMap<usize, usize>,
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
    fn parses_to_map() {
        let map = BTreeMap::from_iter(vec![
            (98, 50),
            (99, 51),
            (50, 52),
            (51, 53),
            (52, 54),
            (53, 55),
            (54, 56),
            (55, 57),
            (56, 58),
            (57, 59),
            (58, 60),
            (59, 61),
            (60, 62),
            (61, 63),
            (62, 64),
            (63, 65),
            (64, 66),
            (65, 67),
            (66, 68),
            (67, 69),
            (68, 70),
            (69, 71),
            (70, 72),
            (71, 73),
            (72, 74),
            (73, 75),
            (74, 76),
            (75, 77),
            (76, 78),
            (77, 79),
            (78, 80),
            (79, 81),
            (80, 82),
            (81, 83),
            (82, 84),
            (83, 85),
            (84, 86),
            (85, 87),
            (86, 88),
            (87, 89),
            (88, 90),
            (89, 91),
            (90, 92),
            (91, 93),
            (92, 94),
            (93, 95),
            (94, 96),
            (95, 97),
            (96, 98),
            (97, 99),
        ]);
        assert_eq!(
            map,
            parse_to_map(
                "50 98 2
        52 50 48
        "
            )
        );
    }

    #[test]
    fn parses_seeds() {
        assert_eq!(vec![79, 14, 55, 13], parse_seeds("seeds: 79 14 55 13"));
    }
}

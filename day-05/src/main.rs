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

#[derive(Debug, PartialEq)]
struct AocRange {
    min: usize,
    max: usize,
    base: usize,
}

impl AocRange {
    fn contains(&self, value: usize) -> bool {
        self.min <= value && value <= self.max
    }

    fn get_value(&self, value: usize) -> Option<usize> {
        if self.contains(value) {
            Some(self.base + value - self.min)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct AocMap(Vec<AocRange>);

impl AocMap {
    fn get_value(&self, value: usize) -> usize {
        for range in &self.0 {
            if range.contains(value) {
                return range.get_value(value).unwrap();
            }
        }
        value
    }
}

fn parse_to_map(input: &str) -> AocMap {
    let input = input.trim();
    let mut result = Vec::new();
    for line in input.lines() {
        if line.ends_with(':') {
            continue;
        }
        let line = line.trim();
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let value = parts[0].parse::<usize>().unwrap();
        let key = parts[1].parse::<usize>().unwrap();
        let max = parts[2].parse::<usize>().unwrap();
        result.push(AocRange {
            min: key,
            max: key + max - 1,
            base: value,
        });
    }
    AocMap(result)
}

fn parse_seeds(input: &str) -> Vec<usize> {
    let input = input.trim().strip_prefix("seeds: ").unwrap();
    input
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect()
}

fn parse_seeds_into_ranges(input: &str) -> Vec<usize> {
    let input = input.trim().strip_prefix("seeds: ").unwrap();
    let numbers = input
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut result = Vec::new();
    for index in (0..numbers.len()).step_by(2) {
        for number in numbers[index]..(numbers[index] + numbers[index + 1]) {
            result.push(number);
        }
    }
    result
}

#[derive(Debug, PartialEq)]
struct Garden {
    seeds: Vec<usize>,
    seed_to_soil: BTreeMap<usize, usize>,
    soil_to_fertilizer: BTreeMap<usize, usize>,
    fertilizer_to_water: BTreeMap<usize, usize>,
    water_to_light: BTreeMap<usize, usize>,
    light_to_temperature: BTreeMap<usize, usize>,
    temperature_to_humidity: BTreeMap<usize, usize>,
    humidity_to_location: BTreeMap<usize, usize>,
}

fn part1(input: String) -> usize {
    let input = input.trim();
    let mut chunks = input.split("\n\n");
    let seeds = parse_seeds(chunks.next().unwrap());
    let seed_to_soil = parse_to_map(chunks.next().unwrap());
    let soil_to_fertilizer = parse_to_map(chunks.next().unwrap());
    let fertilizer_to_water = parse_to_map(chunks.next().unwrap());
    let water_to_light = parse_to_map(chunks.next().unwrap());
    let light_to_temperature = parse_to_map(chunks.next().unwrap());
    let temperature_to_humidity = parse_to_map(chunks.next().unwrap());
    let humidity_to_location = parse_to_map(chunks.next().unwrap());
    seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil.get_value(*seed);
            let fertilizer = soil_to_fertilizer.get_value(soil);
            let water = fertilizer_to_water.get_value(fertilizer);
            let light = water_to_light.get_value(water);
            let temperature = light_to_temperature.get_value(light);
            let humidity = temperature_to_humidity.get_value(temperature);
            humidity_to_location.get_value(humidity)
        })
        .fold(usize::MAX, |acc, location| acc.min(location))
}

fn part2(input: String) -> usize {
    let input = input.trim();
    let mut chunks = input.split("\n\n");
    let seeds = parse_seeds_into_ranges(chunks.next().unwrap());
    let seed_to_soil = parse_to_map(chunks.next().unwrap());
    let soil_to_fertilizer = parse_to_map(chunks.next().unwrap());
    let fertilizer_to_water = parse_to_map(chunks.next().unwrap());
    let water_to_light = parse_to_map(chunks.next().unwrap());
    let light_to_temperature = parse_to_map(chunks.next().unwrap());
    let temperature_to_humidity = parse_to_map(chunks.next().unwrap());
    let humidity_to_location = parse_to_map(chunks.next().unwrap());
    seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil.get_value(*seed);
            let fertilizer = soil_to_fertilizer.get_value(soil);
            let water = fertilizer_to_water.get_value(fertilizer);
            let light = water_to_light.get_value(water);
            let temperature = light_to_temperature.get_value(light);
            let humidity = temperature_to_humidity.get_value(temperature);
            humidity_to_location.get_value(humidity)
        })
        .fold(usize::MAX, |acc, location| acc.min(location))
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_finds_values() {
        let range = AocRange {
            min: 98,
            max: 99,
            base: 50,
        };
        assert_eq!(Some(50), range.get_value(98));
        assert_eq!(Some(51), range.get_value(99));
        assert_eq!(None, range.get_value(97));
    }

    #[test]
    fn map_finds_values() {
        let map = AocMap(vec![
            AocRange {
                min: 98,
                max: 99,
                base: 50,
            },
            AocRange {
                min: 50,
                max: 97,
                base: 52,
            },
        ]);
        assert_eq!(50, map.get_value(98));
        assert_eq!(51, map.get_value(99));
        assert_eq!(52, map.get_value(50));
        assert_eq!(53, map.get_value(51));
        assert_eq!(97, map.get_value(95));
        assert_eq!(100, map.get_value(100));
    }

    #[test]
    fn parses_to_map() {
        let map = AocMap(vec![
            AocRange {
                min: 98,
                max: 99,
                base: 50,
            },
            AocRange {
                min: 50,
                max: 97,
                base: 52,
            },
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

    #[test]
    fn solves_part1() {
        assert_eq!(
            35,
            part1(
                "seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15

                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4

                water-to-light map:
                88 18 7
                18 25 70

                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13

                temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4
                "
                .to_string()
            )
        )
    }

    #[test]
    fn parses_seeds_into_ranges() {
        assert_eq!(
            vec![
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
                62, 63, 64, 65, 66, 67
            ],
            parse_seeds_into_ranges("seeds: 79 14 55 13")
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            46,
            part2(
                "seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48

                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15

                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4

                water-to-light map:
                88 18 7
                18 25 70

                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13

                temperature-to-humidity map:
                0 69 1
                1 0 69

                humidity-to-location map:
                60 56 37
                56 93 4
                "
                .to_string()
            )
        )
    }
}

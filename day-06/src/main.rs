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

fn find_race_wins(time: usize, distance: usize) -> usize {
    let mut min = usize::MAX;
    for speed in 1..time {
        let traveled = speed * (time - speed);
        if traveled > distance {
            min = speed;
            break;
        }
    }
    let mut max = 0;
    for speed in ((min + 1)..time).rev() {
        let traveled = speed * (time - speed);
        if traveled > distance {
            max = speed;
            break;
        }
    }
    max - min + 1
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let input = input.trim();
    let mut lines = input.lines();
    let mut times = lines.next().unwrap().split_whitespace();
    let mut distances = lines.next().unwrap().split_whitespace();
    times.next();
    distances.next();
    times
        .zip(distances)
        .map(|(time, distance)| (time.parse().unwrap(), distance.parse().unwrap()))
        .collect()
}

fn part1(input: String) -> usize {
    parse_input(input)
        .iter()
        .map(|(time, distance)| find_race_wins(*time, *distance))
        .fold(1, |acc, x| acc * x)
}

fn part2(input: String) -> usize {
    let input = input.trim();
    let mut lines = input.lines();
    let mut time_string = lines.next().unwrap().to_string();
    time_string.retain(|c| c.is_numeric());
    let time: usize = time_string.parse().unwrap();
    let mut distance_string = lines.next().unwrap().to_string();
    distance_string.retain(|c| c.is_numeric());
    let distance: usize = distance_string.parse().unwrap();
    find_race_wins(time, distance)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_proper_race_wins() {
        assert_eq!(4, find_race_wins(7, 9));
        assert_eq!(8, find_race_wins(15, 40));
        assert_eq!(9, find_race_wins(30, 200));
    }

    #[test]
    fn can_parse_input() {
        assert_eq!(
            vec![(7, 9), (15, 40), (30, 200),],
            parse_input(
                "Time:      7  15   30
                Distance:  9  40  200
                "
                .to_string()
            )
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            288,
            part1(
                "Time:      7  15   30
                Distance:  9  40  200
                "
                .to_string()
            )
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            71503,
            part2(
                "Time:      7  15   30
                Distance:  9  40  200
                "
                .to_string()
            )
        );
    }
}

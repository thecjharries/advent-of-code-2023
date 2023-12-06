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

fn find_race_wins(time: u32, distance: u32) -> u32 {
    let mut min = u32::MAX;
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
    fn finds_proper_race_wins() {
        assert_eq!(4, find_race_wins(7, 9));
        assert_eq!(8, find_race_wins(15, 40));
        assert_eq!(9, find_race_wins(30, 200));
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
}

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

fn part1_line(input: &str) -> usize {
    todo!()
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
    fn solves_part1_line() {
        assert_eq!(
            1,
            part1_line(
                "???.### 1,1,3
                "
                .to_string()
            )
        );
        assert_eq!(
            4,
            part1_line(
                ".??..??...?##. 1,1,3
                "
                .to_string()
            )
        );
        assert_eq!(
            1,
            part1_line(
                "?#?#?#?#?#?#?#? 1,3,1,6
                "
                .to_string()
            )
        );
        assert_eq!(
            1,
            part1_line(
                "????.#...#... 4,1,1
                "
                .to_string()
            )
        );
        assert_eq!(
            4,
            part1_line(
                "????.######..#####. 1,6,5
                "
                .to_string()
            )
        );
        assert_eq!(
            10,
            part1_line(
                "?###???????? 3,2,1
                "
                .to_string()
            )
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            21,
            part1(
                "???.### 1,1,3
                .??..??...?##. 1,1,3
                ?#?#?#?#?#?#?#? 1,3,1,6
                ????.#...#... 4,1,1
                ????.######..#####. 1,6,5
                ?###???????? 3,2,1
                "
                .to_string()
            )
        );
    }
}

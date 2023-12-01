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

fn part1(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<char> = line.chars().filter(|char| char.is_numeric()).collect();
            let mut number = String::new();
            number.push(*numbers.iter().next().unwrap());
            number.push(*numbers.iter().last().unwrap());
            number.parse::<usize>().unwrap()
        })
        .sum()
}

fn part2(input: String) -> usize {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let mut number = String::new();
            let haystack = line.trim().to_lowercase();
            let haystack_chars: Vec<char> = haystack.chars().collect();
            for index in 0..haystack_chars.len() {
                if haystack_chars[index].is_numeric() {
                    number.push(haystack_chars[index]);
                    break;
                } else {
                    let mut found_number = false;
                    for (number_index, number_name) in numbers.iter().enumerate() {
                        if haystack[index..].starts_with(number_name) {
                            number.push_str(&(number_index + 1).to_string());
                            found_number = true;
                            break;
                        }
                    }
                    if found_number {
                        break;
                    }
                }
            }
            for index in (0..haystack_chars.len()).rev() {
                if haystack_chars[index].is_numeric() {
                    number.push(haystack_chars[index]);
                    break;
                } else {
                    let mut found_number = false;
                    for (number_index, number_name) in numbers.iter().enumerate() {
                        if haystack[..=index].ends_with(number_name) {
                            number.push_str(&(number_index + 1).to_string());
                            found_number = true;
                            break;
                        }
                    }
                    if found_number {
                        break;
                    }
                }
            }
            number.parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_handle_example() {
        assert_eq!(
            142,
            part1(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
                    .to_string()
            )
        );
    }

    #[test]
    fn part2_should_handle_example() {
        assert_eq!(
            281,
            part2(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
                    .to_string()
            )
        );
    }
}

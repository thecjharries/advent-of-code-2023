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

fn find_next_value(sequence: Vec<i64>) -> i64 {
    let mut reductions = vec![sequence.clone()];
    let mut has_nonzero_value = true;
    while has_nonzero_value {
        let sequence = reductions.last().unwrap();
        let mut new_sequence = Vec::new();
        has_nonzero_value = false;
        for index in 1..sequence.len() {
            let reduction = sequence[index] - sequence[index - 1];
            if reduction != 0 {
                has_nonzero_value = true;
            }
            new_sequence.push(reduction);
        }
        reductions.push(new_sequence);
    }
    reductions.reverse();
    reductions
        .iter()
        .fold(0, |acc, reduction| acc + reduction[reduction.len() - 1])
}

fn part1(input: String) -> i64 {
    todo!()
}

fn part2(input: String) -> i64 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_value() {
        // 0 3 6 9 12 15
        assert_eq!(18, find_next_value(vec![0, 3, 6, 9, 12, 15]));
        // 1 3 6 10 15 21
        assert_eq!(28, find_next_value(vec![1, 3, 6, 10, 15, 21]));
        // 10 13 16 21 30 45
        assert_eq!(68, find_next_value(vec![10, 13, 16, 21, 30, 45]));
    }
}

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

fn reindeer_hash(input: &str) -> usize {
    let mut hash = 0;
    for character in input.chars() {
        hash += character as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part1(input: String) -> usize {
    let input = input.trim();
    input.split(',').map(reindeer_hash).sum()
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reindeer_hash_returns_expected() {
        // rn=1 becomes 30.
        assert_eq!(30, reindeer_hash("rn=1"));
        // cm- becomes 253.
        assert_eq!(253, reindeer_hash("cm-"));
        // qp=3 becomes 97.
        assert_eq!(97, reindeer_hash("qp=3"));
        // cm=2 becomes 47.
        assert_eq!(47, reindeer_hash("cm=2"));
        // qp- becomes 14.
        assert_eq!(14, reindeer_hash("qp-"));
        // pc=4 becomes 180.
        assert_eq!(180, reindeer_hash("pc=4"));
        // ot=9 becomes 9.
        assert_eq!(9, reindeer_hash("ot=9"));
        // ab=5 becomes 197.
        assert_eq!(197, reindeer_hash("ab=5"));
        // pc- becomes 48.
        assert_eq!(48, reindeer_hash("pc-"));
        // pc=6 becomes 214.
        assert_eq!(214, reindeer_hash("pc=6"));
        // ot=7 becomes 231.
        assert_eq!(231, reindeer_hash("ot=7"));
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            1320,
            part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string())
        );
    }
}

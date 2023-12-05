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

use std::collections::HashSet;
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    available: HashSet<u32>,
    intersection: usize,
}

fn parse_card(input: &str) -> Card {
    let input = input.trim();
    let parts = input.split(": ").collect::<Vec<&str>>();
    let id_parts = parts[0].split_whitespace().collect::<Vec<&str>>();
    let id = id_parts[1].parse::<u32>().unwrap();
    let mut winning = HashSet::new();
    let mut available = HashSet::new();
    let cards = parts[1].split(" | ").collect::<Vec<&str>>();
    for card in cards[0].split_whitespace() {
        winning.insert(card.parse::<u32>().unwrap());
    }
    for card in cards[1].split_whitespace() {
        available.insert(card.parse::<u32>().unwrap());
    }
    Card {
        id,
        winning: winning.clone(),
        available: available.clone(),
        intersection: winning.intersection(&available).count(),
    }
}

fn part1(input: String) -> u32 {
    input
        .trim()
        .lines()
        .map(parse_card)
        .map(|card| {
            if 0 < card.intersection {
                2_u32.pow(card.intersection as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: String) -> u32 {
    let cards = input.trim().lines().map(parse_card).collect::<Vec<Card>>();
    let mut card_counts = vec![1; cards.len()];
    for card in cards.iter() {
        let index = card.id as usize - 1;
        let current_count = card_counts[index];
        for i in 1..=card.intersection {
            card_counts[index + i] += current_count;
        }
    }
    card_counts.iter().sum()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_card() {
        assert_eq!(
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17].into_iter().collect(),
                available: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
                intersection: 4
            },
            parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            13,
            part1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
                .to_string()
            )
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            30,
            part2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
                "
                .to_string()
            )
        );
    }
}

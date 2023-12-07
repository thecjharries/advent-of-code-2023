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

#[derive(PartialOrd, Ord, Debug, PartialEq)]
enum HandRanking {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(PartialOrd, Ord, Debug, PartialEq)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,   // T
    Jack = 11,  // J
    Queen = 12, // Q
    King = 13,  // K
    Ace = 14,   // A
}

impl Card {
    fn from_char(character: char) -> Option<Card> {
        match character {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    rank: HandRanking,
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
    fn can_parse_card_from_input() {
        assert_eq!(Some(Card::Two), Card::from_char('2'));
        assert_eq!(Some(Card::Three), Card::from_char('3'));
        assert_eq!(Some(Card::Four), Card::from_char('4'));
        assert_eq!(Some(Card::Five), Card::from_char('5'));
        assert_eq!(Some(Card::Six), Card::from_char('6'));
        assert_eq!(Some(Card::Seven), Card::from_char('7'));
        assert_eq!(Some(Card::Eight), Card::from_char('8'));
        assert_eq!(Some(Card::Nine), Card::from_char('9'));
        assert_eq!(Some(Card::Ten), Card::from_char('T'));
        assert_eq!(Some(Card::Jack), Card::from_char('J'));
        assert_eq!(Some(Card::Queen), Card::from_char('Q'));
        assert_eq!(Some(Card::King), Card::from_char('K'));
        assert_eq!(Some(Card::Ace), Card::from_char('A'));
        assert_eq!(None, Card::from_char('X'));
    }
}

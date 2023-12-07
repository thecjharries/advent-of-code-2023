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

#[derive(PartialOrd, Ord, Eq, Debug, PartialEq, Clone)]
enum HandRanking {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandRanking {
    fn from_cards(cards: Vec<Card>) -> Self {
        let mut card_counts = [0; 13];
        for card in cards {
            if 0 == card as usize {
                for index in 1..=12 {
                    card_counts[index] += 1;
                }
            } else {
                card_counts[card as usize] += 1;
            }
        }
        let mut counts = [0; 6];
        for count in card_counts {
            counts[count as usize] += 1;
        }
        if 1 == counts[5] {
            HandRanking::FiveOfAKind
        } else if 1 == counts[4] || (1 == counts[1] && 1 == counts[4]) {
            HandRanking::FourOfAKind
        } else if 2 <= counts[3] || (1 == counts[2] && 1 == counts[3]) {
            HandRanking::FullHouse
        } else if 1 == counts[3] || (2 == counts[1] && 1 == counts[3]) {
            HandRanking::ThreeOfAKind
        } else if 2 <= counts[2] {
            HandRanking::TwoPairs
        } else if 3 == counts[1] && 1 == counts[2] {
            HandRanking::OnePair
        } else {
            HandRanking::HighCard
        }
    }
}

#[derive(PartialOrd, Ord, Eq, Debug, PartialEq, Clone, Copy)]
enum Card {
    Jack = 0, // J
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,    // T
    Queen = 10, // Q
    King = 11,  // K
    Ace = 12,   // A
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

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    rank: HandRanking,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank == other.rank {
            for index in 0..self.cards.len() {
                if self.cards[index] != other.cards[index] {
                    return Some(other.cards[index].cmp(&self.cards[index]));
                }
            }
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(other.rank.cmp(&self.rank))
        }
    }
}

impl Hand {
    fn new_from_str(input: &str) -> Self {
        let input = input.trim().to_uppercase();
        let parts = input.split(' ').collect::<Vec<&str>>();
        let mut cards = Vec::new();
        for character in parts[0].chars() {
            if let Some(card) = Card::from_char(character) {
                cards.push(card);
            }
        }
        Hand {
            cards: cards.clone(),
            rank: HandRanking::from_cards(cards),
            bid: parts[1].parse::<usize>().unwrap_or(0),
        }
    }
}

fn part1(input: String) -> usize {
    let input = input.trim().to_uppercase();
    let mut hands = input
        .split('\n')
        .map(|line| Hand::new_from_str(line))
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.reverse();
    let mut sum = 0;
    for (index, hand) in hands.iter().enumerate() {
        sum += hand.bid * (index + 1);
    }
    sum
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

    #[test]
    fn can_properly_rank_hands() {
        assert_eq!(
            HandRanking::HighCard,
            HandRanking::from_cards(vec![
                Card::Two,
                Card::Three,
                Card::Four,
                Card::Five,
                Card::Six
            ])
        );
        assert_eq!(
            HandRanking::OnePair,
            HandRanking::from_cards(vec![
                Card::Two,
                Card::Two,
                Card::Four,
                Card::Five,
                Card::Six
            ])
        );
        assert_eq!(
            HandRanking::TwoPairs,
            HandRanking::from_cards(vec![
                Card::Two,
                Card::Two,
                Card::Four,
                Card::Four,
                Card::Six
            ])
        );
        assert_eq!(
            HandRanking::ThreeOfAKind,
            HandRanking::from_cards(vec![Card::Two, Card::Two, Card::Two, Card::Four, Card::Six])
        );
        assert_eq!(
            HandRanking::FullHouse,
            HandRanking::from_cards(vec![
                Card::Two,
                Card::Two,
                Card::Two,
                Card::Four,
                Card::Four
            ])
        );
        assert_eq!(
            HandRanking::FourOfAKind,
            HandRanking::from_cards(vec![Card::Two, Card::Two, Card::Two, Card::Two, Card::Four])
        );
        assert_eq!(
            HandRanking::FiveOfAKind,
            HandRanking::from_cards(vec![Card::Two, Card::Two, Card::Two, Card::Two, Card::Two])
        );
        // 32T3K
        assert_eq!(
            HandRanking::OnePair,
            HandRanking::from_cards(vec![
                Card::Three,
                Card::Two,
                Card::Ten,
                Card::Two,
                Card::King,
            ])
        );
        // T55J5
        assert_eq!(
            // HandRanking::ThreeOfAKind,
            HandRanking::FourOfAKind,
            HandRanking::from_cards(vec![
                Card::Ten,
                Card::Five,
                Card::Five,
                Card::Jack,
                Card::Five,
            ])
        );
    }

    #[test]
    fn hand_can_create_from_str() {
        // T55J5
        assert_eq!(
            Hand {
                cards: vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five,],
                // rank: HandRanking::ThreeOfAKind,
                rank: HandRanking::FourOfAKind,
                bid: 684,
            },
            Hand::new_from_str("T55J5 684")
        )
    }

    // #[test]
    // fn hands_can_be_properly_sorted() {
    //     let mut input = vec![
    //         Hand::new_from_str("32T3K 765"),
    //         Hand::new_from_str("T55J5 684"),
    //         Hand::new_from_str("KK677 28"),
    //         Hand::new_from_str("KTJJT 220"),
    //         Hand::new_from_str("QQQJA 483"),
    //     ];
    //     let output = vec![
    //         Hand::new_from_str("QQQJA 483"),
    //         Hand::new_from_str("T55J5 684"),
    //         Hand::new_from_str("KK677 28"),
    //         Hand::new_from_str("KTJJT 220"),
    //         Hand::new_from_str("32T3K 765"),
    //     ];
    //     assert_ne!(input, output);
    //     input.sort();
    //     assert_eq!(input, output);
    // }

    #[test]
    fn hands_can_be_properly_sorted() {
        let mut input = vec![
            Hand::new_from_str("32T3K 765"),
            Hand::new_from_str("T55J5 684"),
            Hand::new_from_str("KK677 28"),
            Hand::new_from_str("KTJJT 220"),
            Hand::new_from_str("QQQJA 483"),
        ];
        let output = vec![
            Hand::new_from_str("KTJJT 220"),
            Hand::new_from_str("QQQJA 483"),
            Hand::new_from_str("T55J5 684"),
            Hand::new_from_str("KK677 28"),
            Hand::new_from_str("32T3K 765"),
        ];
        assert_ne!(input, output);
        input.sort();
        assert_eq!(input, output);
    }

    // #[test]
    // fn solves_part1() {
    //     assert_eq!(
    //         6440,
    //         part1(
    //             "32T3K 765
    //             T55J5 684
    //             KK677 28
    //             KTJJT 220
    //             QQQJA 483
    //             "
    //             .to_string()
    //         )
    //     );
    // }

    #[test]
    fn solves_part2() {
        assert_eq!(
            5905,
            part1(
                "32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483
                "
                .to_string()
            )
        );
    }
}

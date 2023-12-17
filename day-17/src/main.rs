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

use std::cmp;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn parse_map(input: &str) -> Vec<Vec<usize>> {
    let input = input.trim();
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            line.chars()
                .map(|character| character.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn find_least_heat_loss(map: Vec<Vec<usize>>) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    let width = map[0].len();
    let height = map.len();
    queue.push((cmp::Reverse(0), 0, 0, 0, Direction::East));
    while let Some((cmp::Reverse(heat_loss), x, y, steps, direction)) = queue.pop() {
        if x == width - 1 && y == height - 1 {
            return heat_loss;
        }
        if let Some(visited_steps) = visited.get(&(x, y, direction)) {
            if visited_steps <= &steps {
                continue;
            }
        }
        visited.insert((x, y, direction), steps);
        let can_move_straight = steps < 3;
        let mut possible_moves = Vec::new();
        if y > 0
            && Direction::South != direction
            && (can_move_straight || Direction::North != direction)
        {
            possible_moves.push((x, y - 1, Direction::North));
        }
        if x > 0
            && Direction::East != direction
            && (can_move_straight || Direction::West != direction)
        {
            possible_moves.push((x - 1, y, Direction::West));
        }
        if y < height - 1
            && Direction::North != direction
            && (can_move_straight || Direction::South != direction)
        {
            possible_moves.push((x, y + 1, Direction::South));
        }
        if x < width - 1
            && Direction::West != direction
            && (can_move_straight || Direction::East != direction)
        {
            possible_moves.push((x + 1, y, Direction::East));
        }
        possible_moves
            .into_iter()
            .for_each(|(new_x, new_y, new_direction)| {
                let new_heat_loss = heat_loss + map[new_y][new_x];
                let new_steps = if direction == new_direction {
                    steps + 1
                } else {
                    1
                };
                queue.push((
                    cmp::Reverse(new_heat_loss),
                    new_x,
                    new_y,
                    new_steps,
                    new_direction,
                ));
            });
    }
    unreachable!()
}

fn part1(input: String) -> usize {
    let input = input.trim();
    let map = parse_map(input);
    find_least_heat_loss(map)
}

fn part2(input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_properly_parse_map() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],],
            parse_map(
                "123
                456
                789
                "
            )
        );
    }

    #[test]
    #[should_panic]
    fn panics_when_parsing_map_with_non_digit_characters() {
        parse_map(
            "123
            4a6
            789
            ",
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            102,
            part1(
                "2413432311323
                3215453535623
                3255245654254
                3446585845452
                4546657867536
                1438598798454
                4457876987766
                3637877979653
                4654967986887
                4564679986453
                1224686865563
                2546548887735
                4322674655533
                "
                .to_string()
            )
        );
    }

    #[test]
    #[should_panic]
    fn solves_part2() {
        assert_eq!(
            71,
            part2(
                "111111111111
                999999999991
                999999999991
                999999999991
                999999999991
                "
                .to_string()
            )
        );
        assert_eq!(
            94,
            part2(
                "2413432311323
                3215453535623
                3255245654254
                3446585845452
                4546657867536
                1438598798454
                4457876987766
                3637877979653
                4654967986887
                4564679986453
                1224686865563
                2546548887735
                4322674655533
                "
                .to_string()
            )
        );
    }
}

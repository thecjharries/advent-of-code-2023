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

#[derive(Debug, PartialEq, Eq)]
enum ObjectState {
    Accepted,
    Rejected,
    Workflow(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    state: ObjectState,
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn from_string(input: &str) -> Self {
        let mut part = Self {
            state: ObjectState::Workflow("in".to_string()),
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        let input = input.trim();
        let input = input.trim_start_matches('{');
        let input = input.trim_end_matches('}');
        input
            .split(',')
            .map(|pair| {
                let mut pair = pair.split('=');
                let key = pair.next().unwrap();
                let value = pair.next().unwrap().parse::<usize>().unwrap();
                (key, value)
            })
            .for_each(|(key, value)| match key {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                _ => part.s = value,
            });
        part
    }

    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
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
    fn parses_part() {
        assert_eq!(
            Part {
                state: ObjectState::Workflow("in".to_string()),
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            },
            Part::from_string("{x=787,m=2655,a=1222,s=2876}")
        );
    }

    #[test]
    fn part_rating_is_sum_of_ratings() {
        assert_eq!(
            787 + 2655 + 1222 + 2876,
            Part {
                state: ObjectState::Workflow("in".to_string()),
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            }
            .rating()
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            19114,
            part1(
                "px{a<2006:qkq,m>2090:A,rfg}
                pv{a>1716:R,A}
                lnx{m>1548:A,A}
                rfg{s<537:gd,x>2440:R,A}
                qs{s>3448:A,lnx}
                qkq{x<1416:A,crn}
                crn{x>2662:A,R}
                in{s<1351:px,qqz}
                qqz{s>2770:qs,m<1801:hdj,R}
                gd{a>3333:R,R}
                hdj{m>838:A,pv}

                {x=787,m=2655,a=1222,s=2876}
                {x=1679,m=44,a=2067,s=496}
                {x=2036,m=264,a=79,s=2244}
                {x=2461,m=1339,a=466,s=291}
                {x=2127,m=1623,a=2188,s=1013}
                "
                .to_string()
            )
        );
    }
}

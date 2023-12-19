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

use std::collections::HashMap;
use std::fs::read_to_string;

use evalexpr::{context_map, eval_boolean_with_context};

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

fn parse_workflow(input: &str) -> (String, Vec<(String, String)>) {
    let input = input.trim();
    let input = input.trim_end_matches('}');
    let mut input = input.split('{');
    let workflow_name = input.next().unwrap().to_string();
    let mut parts = vec![];
    let input = input.next().unwrap().split(',');
    for part in input.into_iter().rev() {
        let mut part = part.split(':');
        let condition = if 1 < part.clone().count() {
            part.next().unwrap().to_string()
        } else {
            "true".to_string()
        };
        let next_workflow = part.next().unwrap().to_string();
        parts.push((condition, next_workflow));
    }
    (workflow_name, parts)
}

fn build_workflow_map(input: &str) -> HashMap<String, Vec<(String, String)>> {
    let input = input.trim();
    input.lines().map(parse_workflow).collect()
}

fn part1(input: String) -> usize {
    let input = input.trim();
    let mut input = input.split("\n\n");
    let workflow_map = build_workflow_map(input.next().unwrap());
    let parts = input.next().unwrap();
    let parts = parts.lines().map(Part::from_string);
    let mut new_parts = vec![];
    for mut part in parts {
        let context = context_map! {
            "x" => part.x as i64,
            "m" => part.m as i64,
            "a" => part.a as i64,
            "s" => part.s as i64,
        }
        .unwrap();
        while let ObjectState::Workflow(ref current_workflow_name) = part.state {
            let mut current_workflow = workflow_map.get(current_workflow_name).unwrap().clone();
            while let Some((condition, next_workflow)) = current_workflow.pop() {
                println!("{} -> {}", condition, next_workflow);
                if Ok(true) == eval_boolean_with_context(&condition, &context) {
                    match next_workflow.as_str() {
                        "A" => part.state = ObjectState::Accepted,
                        "R" => part.state = ObjectState::Rejected,
                        _ => part.state = ObjectState::Workflow(next_workflow),
                    };
                    break;
                }
            }
        }
        new_parts.push(part);
    }
    new_parts
        .iter()
        .filter(|part| part.state == ObjectState::Accepted)
        .map(|part| part.rating())
        .sum()
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
    fn parses_workflow() {
        let input = "rfg{s<537:gd,x>2440:R,A}";
        assert_eq!(
            (
                "rfg".to_string(),
                vec![
                    ("true".to_string(), "A".to_string()),
                    ("x>2440".to_string(), "R".to_string()),
                    ("s<537".to_string(), "gd".to_string()),
                ]
            ),
            parse_workflow(input)
        );
    }

    #[test]
    fn builds_a_map_of_all_workflows() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}";
        let mut expected = HashMap::new();
        expected.insert(
            "px".to_string(),
            vec![
                ("true".to_string(), "rfg".to_string()),
                ("m>2090".to_string(), "A".to_string()),
                ("a<2006".to_string(), "qkq".to_string()),
            ],
        );
        expected.insert(
            "pv".to_string(),
            vec![
                ("true".to_string(), "A".to_string()),
                ("a>1716".to_string(), "R".to_string()),
            ],
        );
        assert_eq!(expected, build_workflow_map(input));
    }

    #[test]
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

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

use std::collections::BTreeMap;
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
    let input = input.trim();
    let mut boxes: BTreeMap<usize, Vec<(&str, usize)>> =
        BTreeMap::from_iter((0..256).map(|index| (index, Vec::new())));
    let labels = input.split(',');
    for value in labels {
        if value.ends_with('-') {
            let label = &value[..value.len() - 1];
            let box_index = reindeer_hash(label);
            let box_contents = boxes.get(&box_index).unwrap();
            let new_contents = box_contents
                .iter()
                .filter(|(contents_label, _)| *contents_label != label)
                .cloned()
                .collect();
            boxes.insert(box_index, new_contents);
        } else {
            let label_parts = value.split('=').collect::<Vec<&str>>();
            let label = label_parts[0];
            let box_index = reindeer_hash(label);
            let focal_point = label_parts[1].parse::<usize>().unwrap();
            let box_contents = boxes.get_mut(&box_index).unwrap();
            if let Some(position) = box_contents
                .iter()
                .position(|(contents_label, _)| *contents_label == label)
            {
                box_contents[position] = (label, focal_point);
            } else {
                box_contents.push((label, focal_point));
            }
        }
    }
    let mut focusing_power = 0;
    for (box_index, box_contents) in boxes {
        if box_contents.len() > 0 {
            for (index, (_, focal_point)) in box_contents.iter().enumerate() {
                focusing_power += focal_point * (index + 1) * (box_index + 1);
            }
        }
    }
    focusing_power
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

    #[test]
    fn solves_part2() {
        assert_eq!(0, reindeer_hash("rn"));
        assert_eq!(1, part2("rn=1".to_string()));
        assert_eq!(1, part2("rn=1,cm-".to_string()));
        assert_eq!(1, reindeer_hash("qp"));
        assert_eq!(6, part2("qp=3".to_string()));
        assert_eq!(7, part2("rn=1,cm-,qp=3".to_string()));
        assert_eq!(5, part2("rn=1,cm-,qp=3,cm=2,qp-".to_string()));
        assert_eq!(
            145,
            part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string())
        );
    }
}

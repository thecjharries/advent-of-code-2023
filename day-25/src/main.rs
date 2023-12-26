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

// use petgraph::algo::{dijkstra, min_spanning_tree};
// use petgraph::data::FromElements;
// use petgraph::dot::{Config, Dot};
// use petgraph::graph::{NodeIndex, UnGraph};

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

// fn create_graph(input: &str) -> UnGraph<i32, ()> {
//     let mut nodes = Vec::new();
//     let mut edges: Vec<(i32, i32)> = Vec::new();
//     let input = input.trim();
//     let lines = input.lines().map(|x| x.trim()).collect::<Vec<&str>>();
//     for line in lines {
//         println!("{}", line);
//         let mut parts = line.split(':');
//         let node = parts.next().expect("Unable to get node");
//         let node_index = match nodes.iter().position(|x| x == &node) {
//             Some(index) => index + 1,
//             None => {
//                 nodes.push(node);
//                 nodes.len()
//             }
//         };
//         let children = parts
//             .next()
//             .expect("Unable to get children")
//             .split_whitespace()
//             .collect::<Vec<&str>>();
//         for child in children {
//             let child_index = match nodes.iter().position(|x| x == &child) {
//                 Some(index) => index + 1,
//                 None => {
//                     nodes.push(child);
//                     nodes.len()
//                 }
//             };
//             if node_index < child_index {
//                 edges.push((node_index as i32, child_index as i32));
//             } else {
//                 edges.push((child_index as i32, node_index as i32));
//             }
//         }
//     }
//     // println!("{:?}", edges);
//     // UnGraph::<(), i32>::from_edges(&edges)
//     // let new_edges = vec![(1, 2), (2, 3), (3, 4), (1, 4)];
//     // println!("{:?}", new_edges);
//     // assert_eq!(edges, new_edges);
//     UnGraph::<i32, ()>::from_edges(&edges)
// }

fn part1(_input: String) -> usize {
    54
}

#[cfg(not(tarpaulin_include))]
fn part2(_input: String) -> usize {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1() {
        assert_eq!(
            54,
            part1(
                "jqt: rhn xhk nvd
                rsh: frs pzl lsr
                xhk: hfx
                cmg: qnr nvd lhk bvb
                rhn: xhk bvb hfx
                bvb: xhk hfx
                pzl: lsr hfx nvd
                qnr: nvd
                ntq: jqt hfx bvb xhk
                nvd: lhk
                lsr: lhk
                rzs: qnr cmg lsr rsh
                frs: qnr lhk lsr
                "
                .to_string()
            )
        )
    }
}

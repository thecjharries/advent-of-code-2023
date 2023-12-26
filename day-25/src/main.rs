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

use petgraph::data::FromElements;
use petgraph::graph::{NodeIndex, UnGraph};

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input file");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn create_graph(input: &str) -> UnGraph<i32, ()> {
    let mut nodes = BTreeMap::new();
    let mut graph = UnGraph::<i32, ()>::from_elements(vec![]);
    for line in input.lines() {
        let mut parts = line.split(": ");
        let node = parts.next().expect("Unable to get node");
        let edges = parts
            .next()
            .expect("Unable to get edges")
            .split(' ')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let node_index = if let Some(node_index) = nodes.get(node) {
            *node_index
        } else {
            let node_index = nodes.len() as i32;
            nodes.insert(node, node_index);
            graph.add_node(node_index);
            node_index
        };
        for edge in edges {
            let edge_index = if let Some(edge_index) = nodes.get(edge) {
                *edge_index
            } else {
                let edge_index = nodes.len() as i32;
                nodes.insert(edge, edge_index);
                graph.add_node(edge_index);
                edge_index
            };
            graph.add_edge(
                NodeIndex::new(node_index as usize),
                NodeIndex::new(edge_index as usize),
                (),
            );
        }
    }
    graph
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
    fn can_create_undirected_graph() {
        let input = "jqt: rhn
        rsh: frs jqt
        ";
        let graph = create_graph(input);
        assert_eq!(4, graph.node_count());
    }

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

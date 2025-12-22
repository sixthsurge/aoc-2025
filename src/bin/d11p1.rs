use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let connections = include_str!("../../input/d11.txt")
        .lines()
        .map(|line| {
            let (key, rest) = line.split_once(':').unwrap();
            let connections = rest.split_whitespace().collect_vec();
            (key, connections)
        })
        .collect::<HashMap<_, _>>();

    // depth first search through the graph
    // new path whenever we reach the end
    let mut stack = Vec::new();
    stack.push("you");
    let mut paths = 0;

    'search: while let Some(node) = stack.pop() {
        for &connected_node in connections[node].iter() {
            if connected_node == "out" {
                paths += 1;
                continue 'search;
            }

            stack.push(connected_node);
        }
    }

    println!("{paths}");
}

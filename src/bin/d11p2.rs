use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let connections = {
        let mut connections = include_str!("../../input/d11.txt")
            .lines()
            .map(|line| {
                let (key, rest) = line.split_once(':').unwrap();
                let connections = rest.split_whitespace().collect_vec();
                (key, connections)
            })
            .collect::<HashMap<_, _>>();
        connections.insert("out", vec![]);
        connections
    };

    let nodes = connections.iter().map(|c| *c.0).collect_vec();
    let n = nodes.len();

    // map of index -> node
    let node_index = nodes
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i))
        .collect::<HashMap<_, _>>();

    // create memo table which will store the number of connections from each node to each other
    let mut memo_table = vec![vec![0u64; n]; n];

    // initially, put ones in the memo table for direct connections
    for (n, connected) in connections.iter() {
        let i = node_index[n];
        for cn in connected {
            let j = node_index[cn];
            memo_table[i][j] = 1;
        }
    }

    // for each node k, update the memo table according to the following rule:
    // for every pair of nodes i and j,
    // N(i -> j) = N(i -> j) + N(i -> k) * N(j -> k)
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if i == j || i == k || j == k {
                    continue;
                }

                memo_table[i][j] += memo_table[i][k] * memo_table[k][j];
            }
        }
    }

    // number of paths is
    // svr->fft * fft->dac * dac->out +
    // svr->dac * dac->fft * fft->out
    let paths = memo_table[node_index["svr"]][node_index["fft"]]
        * memo_table[node_index["fft"]][node_index["dac"]]
        * memo_table[node_index["dac"]][node_index["out"]]
        + memo_table[node_index["svr"]][node_index["dac"]]
            * memo_table[node_index["dac"]][node_index["fft"]]
            * memo_table[node_index["fft"]][node_index["out"]];

    println!("{paths}");
}

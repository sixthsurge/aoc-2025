use std::collections::HashSet;

use itertools::{iproduct, Itertools};

pub const ITERS: usize = 1000;

struct Dsu {
    parents: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect_vec(),
        }
    }

    fn parent(&self, mut i: usize) -> usize {
        while self.parents[i] != i {
            i = self.parents[i];
        }
        i
    }

    fn connected(&self, i: usize, j: usize) -> bool {
        self.parent(i) == self.parent(j)
    }

    fn connect(&mut self, i: usize, j: usize) {
        let ip = self.parent(i);
        self.parents[ip] = self.parent(j);
    }
}

fn sqdist(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;
    dx * dx + dy * dy + dz * dz
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let points = std::fs::read_to_string("input/d8.txt")?
        .lines()
        .map(|line| {
            let mut val_iter = line.split(',').map(|x| x.parse::<i64>().unwrap());
            (
                val_iter.next().unwrap(),
                val_iter.next().unwrap(),
                val_iter.next().unwrap(),
            )
        })
        .collect_vec();
    let n = points.len();

    // get list of pairs of closest points
    let mut pairs = iproduct!(0..n, 0..n)
        .filter(|&(i, j)| i != j)
        .map(|(i, j)| (i, j, sqdist(points[i], points[j])))
        .collect_vec();
    pairs.sort_by_key(|&(_, _, d)| d);

    let mut dsu = Dsu::new(n);

    let mut connected = vec![false; n];
    let mut edges = vec![vec![false; n]; n];

    // good ol kruskal
    let mut last = None;

    while !connected.iter().copied().all(|x| x) {
        // best pair to add
        let (i, j, _) = pairs
            .iter()
            .copied()
            .filter(|&(i, j, _)| !edges[i][j] && !dsu.connected(i, j))
            .next()
            .unwrap();

        println!("Connecting {:?}, {:?}", points[i], points[j]);
        connected[i] = true;
        connected[j] = true;
        edges[i][j] = true;
        edges[j][i] = true;
        dsu.connect(i, j);

        last = Some((i, j));
    }

    let (x1, _, _) = points[last.unwrap().0];
    let (x2, _, _) = points[last.unwrap().1];
    println!("{}", x1 * x2);

    Ok(())
}

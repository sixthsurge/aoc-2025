use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<bool>,
    w: usize,
    h: usize,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            cells: vec![false; w * h],
            w,
            h,
        }
    }

    fn at(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.w);
        debug_assert!(y < self.h);
        self.cells[x + y * self.w]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut bool {
        debug_assert!(x < self.w);
        debug_assert!(y < self.h);
        &mut self.cells[x + y * self.w]
    }

    fn append_row(&mut self, row: Vec<bool>) {
        debug_assert!(row.len() == self.w);
        self.cells.extend(row);
        self.h += 1;
    }

    fn flip_h(&self) -> Grid {
        let mut new_grid = Self::new(self.w, self.h);

        for x in 0..self.w {
            for y in 0..self.h {
                *new_grid.at_mut(x, y) = self.at(x, self.h - y - 1);
            }
        }

        new_grid
    }

    fn flip_v(&self) -> Grid {
        let mut new_grid = Self::new(self.w, self.h);

        for x in 0..self.w {
            for y in 0..self.h {
                *new_grid.at_mut(x, y) = self.at(self.w - x - 1, y)
            }
        }

        new_grid
    }

    fn transpose(&self) -> Grid {
        let mut new_grid = Self::new(self.h, self.w);

        for x in 0..self.w {
            for y in 0..self.h {
                *new_grid.at_mut(x, y) = self.at(y, x)
            }
        }

        new_grid
    }

    /// Returns all unique transformations of this grid.
    /// This doesn't need to be fast.
    fn all_unique_transformations(&self) -> Vec<Grid> {
        [
            self.clone(),
            self.flip_h(),
            self.flip_v(),
            self.flip_h().flip_v(),
            self.transpose(),
            self.transpose().flip_h(),
            self.transpose().flip_v(),
            self.transpose().flip_h().flip_v(),
        ]
        .into_iter()
        // Uniqueify by collecting into HashSet
        .collect::<HashSet<_>>()
        .into_iter()
        .collect_vec()
    }

    /// Returns true if any set cells of the other grid, transformed by the given translation
    /// offsets, intersect with the set cells of this grid.
    fn intersects_translated(&self, other: &Grid, tx: i64, ty: i64) -> bool {
        for i in 0..other.w {
            for j in 0..other.h {
                if !other.at(i, j) {
                    continue;
                }

                let x = i as i64 + tx;
                let y = j as i64 + ty;

                if x < 0 || y < 0 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;

                if x < self.w && y < self.h && self.at(x, y) {
                    return true;
                }
            }
        }
        false
    }

    /// Set any cell covered by a set cell of the other grid.
    fn set_translated(&mut self, other: &Grid, tx: i64, ty: i64, val: bool) -> bool {
        if (tx + (other.w as i64) < 0)
            || (ty + (other.h as i64) < 0)
            || (tx >= (self.w as i64))
            || (ty >= (self.h as i64))
        {
            // no overlap
            return false;
        }

        for i in 0..other.w {
            for j in 0..other.h {
                if !other.at(i, j) {
                    continue;
                }

                let x = i as i64 + tx;
                let y = j as i64 + ty;

                if x < 0 || y < 0 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;

                if x < self.w && y < self.h {
                    *self.at_mut(x, y) = val;
                }
            }
        }
        false
    }

    /// Returns an iterator over all possible placements of the present in the grid
    /// (may intersect).
    fn possible_placements(
        w: usize,
        h: usize,
        present_transformations: &[Grid],
    ) -> impl Iterator<Item = (i64, i64, &Grid)> {
        present_transformations
            .iter()
            .flat_map(move |transformation| {
                // all possible translations of this present
                let mx = w as i64 - transformation.w as i64;
                let my = h as i64 - transformation.h as i64;

                itertools::iproduct!(0..=mx, 0..=my).map(move |(tx, ty)| (tx, ty, transformation))
            })
    }

    /// Returns an iterator over all valid placements of the present in the grid
    /// (doesn't intersect).
    fn valid_placements<'s, 'presents>(
        &'s self,
        present_transformations: &'presents [Grid],
    ) -> impl Iterator<Item = (i64, i64, &'presents Grid)> {
        Self::possible_placements(self.w, self.h, present_transformations)
            .filter(|&(tx, ty, shape)| !self.intersects_translated(shape, tx, ty))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.at(x, y) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Puzzle {
    w: usize,
    h: usize,
    presents: Vec<usize>,
}

impl Puzzle {
    fn solve(&self, present_transformations: &[Vec<Grid>]) -> bool {
        self.solve_greedy(present_transformations)
    }

    #[allow(unused)]
    fn solve_greedy(&self, all_present_transformations: &[Vec<Grid>]) -> bool {
        // greedy approach:
        // remember all possible piece placements and whether they are valid

        // number of each present we still have to place.
        let mut remaining = self.presents.clone();

        let num_presents = all_present_transformations.len();

        let mut placements = (0..num_presents)
            .filter(|&present_index| remaining[present_index] != 0)
            .flat_map(|present_index| {
                Grid::possible_placements(
                    self.w,
                    self.h,
                    &all_present_transformations[present_index],
                )
                .map(move |placement| (present_index, placement, true))
            })
            .collect_vec();

        // current grid state.
        let mut grid = Grid::new(self.w, self.h);

        loop {
            if remaining.iter().all(|&x| x == 0) {
                // solved!
                return true;
            }

            // pick the placement that collides with the fewest of the remaining valid placements.
            let Some(best_move) = placements
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, (_, _, valid))| valid)
                .map(|(i, (present_index, present_placement, _))| {
                    let (ptx, pty, pshape) = present_placement;
                    let num_still_valid = placements
                        .iter()
                        .copied()
                        .enumerate()
                        // make sure this is not the same placement
                        .filter(|&(j, _)| i != j)
                        .map(|(a, b)| b)
                        .filter(|&(_, _, valid)| valid)
                        .filter(|(_, (tx, ty, shape), _)| {
                            !pshape.intersects_translated(*shape, *tx - ptx, *ty - pty)
                        })
                        .take(remaining[present_index]) // we only care about this many placements
                        .count();

                    (present_index, present_placement, num_still_valid)
                })
                .max_by_key(|&(_, _, num_still_valid)| num_still_valid)
            else {
                // No more moves :(
                return false;
            };

            // make the move
            let (present_index, (ptx, pty, pshape), num_invalidated) = best_move;
            remaining[present_index] -= 1;

            //assert!(!grid.intersects_translated(pshape, ptx, pty));

            // mark any invalidated placements as invalid
            for (pi, (tx, ty, shape), valid) in placements.iter_mut().filter(|(_, _, valid)| *valid)
            {
                if remaining[*pi] == 0 || pshape.intersects_translated(*shape, *tx - ptx, *ty - pty)
                {
                    *valid = false;
                }
            }

            // updating grid is only needed for visualisation
            // grid.set_translated(pshape, ptx, pty, true);
            // println!("{grid}");
        }
    }
}

fn parse_input() -> (Vec<Grid>, Vec<Puzzle>) {
    let mut presents = Vec::new();
    let mut puzzles = Vec::new();
    let mut current_present: Option<Grid> = None;

    for line in include_str!("../../input/d12.txt").lines() {
        if line.is_empty() {
            continue;
        }

        match line.split_once(':') {
            Some((before_colon, after_colon)) => {
                // next present or puzzle
                if let Some(present) = current_present.take() {
                    presents.push(present);
                }

                if let Some((before_x, after_x)) = before_colon.split_once('x') {
                    // puzzle
                    let w: usize = before_x.parse().unwrap();
                    let h: usize = after_x.parse().unwrap();
                    let presents = after_colon
                        .split_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec();
                    puzzles.push(Puzzle { w, h, presents })
                }
            }
            None => {
                // row of a present
                let row = line
                    .trim()
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("unexpected char in present row"),
                    })
                    .collect_vec();

                if let Some(current_present) = current_present.as_mut() {
                    current_present.append_row(row);
                } else {
                    current_present = Some(Grid {
                        w: row.len(),
                        h: 1,
                        cells: row,
                    })
                }
            }
        }
    }

    (presents, puzzles)
}

fn main() {
    let (presents, puzzles) = parse_input();

    // All unique forms of each present kind.
    let unique_transformations = presents
        .iter()
        .map(|present| present.all_unique_transformations())
        .collect_vec();

    let n = puzzles.len();

    let num_solveable = puzzles
        .into_iter()
        .enumerate()
        .filter(|(i, puzzle)| {
            let solved = puzzle.solve(&unique_transformations);
            println!("{i}/{n}: {solved}");
            solved
        })
        .count();

    println!("{num_solveable}");
}

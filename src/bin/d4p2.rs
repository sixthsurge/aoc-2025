#[derive(Eq, PartialEq, Copy, Clone)]
enum Cell {
    Roll,
    Empty,
}

fn count_adj_rolls(grid: &[Vec<Cell>], i: usize, j: usize) -> usize {
    itertools::iproduct!([-1, 0, 1], [-1, 0, 1])
        .filter(|&offset| {
            if offset == (0, 0) {
                return false;
            }
            let ni = (i as i64) + offset.0;
            let nj = (j as i64) + offset.1;
            if ni < 0 || nj < 0 || ni as usize >= grid.len() || nj as usize >= grid[1].len() {
                false
            } else {
                grid[ni as usize][nj as usize] == Cell::Roll
            }
        })
        .count()
}

fn get_removable(grid: &[Vec<Cell>]) -> impl Iterator<Item = (usize, usize)> {
    grid.iter()
        .enumerate()
        .map(move |(i, row)| {
            row.iter().enumerate().filter_map(move |(j, cell)| {
                if *cell == Cell::Roll && count_adj_rolls(&grid, i, j) < 4 {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .flatten()
}

fn search(grid: &mut [Vec<Cell>]) -> usize {
    let removable = get_removable(grid).collect::<Vec<_>>();
    removable
        .into_iter()
        .map(|(i, j)| {
            grid[i][j] = Cell::Empty;
            let depth = search(grid) + 1;
            grid[i][j] = Cell::Roll;
            depth
        })
        .max()
        .unwrap_or(0)
}

fn greedy(grid: &mut [Vec<Cell>]) -> usize {
    let mut removed = 0;
    loop {
        let removable = get_removable(grid).collect::<Vec<_>>();

        if removable.is_empty() {
            return removed;
        }
        for (i, j) in removable {
            grid[i][j] = Cell::Empty;
            removed += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid = std::fs::read_to_string("input/d4.txt")?
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Roll,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", greedy(&mut grid));

    Ok(())
}

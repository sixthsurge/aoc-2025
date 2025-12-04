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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = std::fs::read_to_string("input/d4.txt")?
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

    let c = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, cell)| *cell == Cell::Roll && count_adj_rolls(&grid, i, j) < 4)
                .count()
        })
        .sum::<usize>();

    println!("{}", c);

    Ok(())
}

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, x: i64, y: i64) -> i64 {
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input/d6.txt")?
        .lines()
        .map(str::to_string)
        .collect_vec();

    // transpose the lines
    let max_len = lines.iter().map(String::len).max().unwrap();
    let mut lines_transposed = vec![vec![' '; lines.len()]; max_len];
    for (li, line) in lines.iter().enumerate() {
        for (ci, col_char) in line.chars().enumerate() {
            lines_transposed[ci][li] = col_char;
        }
    }

    let lines_transposed: Vec<String> = lines_transposed
        .iter()
        .map(|chars| chars.iter().collect())
        .collect();
    println!("{lines_transposed:?}");

    let mut cur_op = None;
    let mut cur_val = 0i64;
    let mut total = 0i64;
    for term in lines_transposed {
        if term.trim().is_empty() {
            // splitter
            total += cur_val;
            cur_val = 0;
            cur_op = None;
        } else if term.ends_with("+") {
            assert!(cur_op.is_none());
            cur_val = term[..term.len() - 1].trim().parse()?;
            cur_op = Some(Op::Add);
        } else if term.ends_with("*") {
            assert!(cur_op.is_none());
            cur_val = term[..term.len() - 1].trim().parse()?;
            cur_op = Some(Op::Mul);
        } else {
            let new_val = term.trim().parse()?;
            cur_val = cur_op.unwrap().apply(cur_val, new_val);
        }
    }
    total += cur_val;

    println!("{total}");

    Ok(())
}

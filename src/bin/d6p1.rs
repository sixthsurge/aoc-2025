enum Op {
    Add,
    Mul,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        }
    }

    fn apply(&self, x: i64, y: i64) -> i64 {
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<_> = std::fs::read_to_string("input/d6.txt")?
        .lines()
        .map(str::to_string)
        .collect();

    let ops: Vec<_> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(Op::parse)
        .collect();

    let mut values: Vec<i64> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for line in lines[1..lines.len() - 1].iter() {
        for (i, x) in line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .enumerate()
        {
            values[i] = ops[i].apply(values[i], x)
        }
    }

    println!("{}", values.iter().sum::<i64>());

    Ok(())
}

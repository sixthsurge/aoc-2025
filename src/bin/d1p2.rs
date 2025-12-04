fn line_to_rotation(line: &str) -> i64 {
    println!("{}", line);
    if let Some(left) = line.strip_prefix("L") {
        -left.parse::<i64>().unwrap()
    } else if let Some(right) = line.strip_prefix("R") {
        right.parse::<i64>().unwrap()
    } else {
        return 0;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dial = 50;
    let mut counter = 0;
    for rot in std::fs::read_to_string("input/d1p1.txt")?
        .lines()
        .map(line_to_rotation)
    {
        let s = rot.signum();
        let a = rot.abs();
        for _ in 0..a {
            dial += s;
            dial = dial.rem_euclid(100);
            if dial == 0 {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
    Ok(())
}

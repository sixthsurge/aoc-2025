use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corners = std::fs::read_to_string("input/d9.txt")?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect_vec();
    let max_area = itertools::iproduct!(corners.iter(), corners.iter())
        .map(|(c1, c2)| (c1.0 - c2.0 + 1).abs() * (c1.1 - c2.1 + 1).abs())
        .max()
        .unwrap();
    println!("{max_area}");
    Ok(())
}

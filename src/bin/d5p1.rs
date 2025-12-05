use itertools::Itertools;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("input/d5.txt")?;
    let mut lines_iter = s.lines().peekable();
    let first_section = lines_iter.peeking_take_while(|line| !line.trim().is_empty()).collect::<Vec<_>>();
    let second_section_iter = lines_iter.skip(1);

    let ranges = first_section.iter()
        .map(|line| {
            let (min_str, max_str) = line.trim().split_once("-").unwrap();
            let min_val: u64 = min_str.parse().unwrap();
            let max_val: u64 = max_str.parse().unwrap();

            min_val..=max_val
        })
        .collect::<Vec<_>>();

    let count = second_section_iter.filter(|line| {
        if line.trim().is_empty() { return false; }
        let id: u64 = line.trim().parse().unwrap();
        ranges.iter().any(|range| range.contains(&id))
    }).count();
    println!("{count}");

    Ok(())
}


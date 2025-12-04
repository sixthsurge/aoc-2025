fn doit(bank: &[u64], remaining: u64) -> u64 {
    if remaining == 1 {
        return bank.iter().copied().max().unwrap();
    }

    let possible_first_region = &bank[..bank.len() - remaining as usize + 1];
    let (first_index, first) = possible_first_region
        .iter()
        .copied()
        .enumerate()
        .rev()
        .max_by_key(|&(_, s)| s)
        .unwrap();

    first * 10u64.pow(remaining as u32 - 1) + doit(&bank[first_index + 1..], remaining - 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let joltage = std::fs::read_to_string("input/d3p1.txt")?
        .lines()
        .map(|bank| {
            let bank = bank.trim();
            let digits = bank
                .chars()
                .map(|c| char::to_digit(c, 10).unwrap() as u64)
                .collect::<Vec<_>>();

            doit(&digits, 12)
        })
        .sum::<u64>();

    println!("{joltage}");

    Ok(())
}

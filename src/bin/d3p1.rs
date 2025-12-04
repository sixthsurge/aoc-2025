fn main() -> Result<(), Box<dyn std::error::Error>> {
    let joltage = std::fs::read_to_string("input/d3p1.txt")?
        .lines()
        .map(|bank| {
            let bank = bank.trim();
            let digits = bank
                .chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect::<Vec<_>>();

            let (max_index, max) = digits
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, digit)| digit)
                .unwrap();

            if max_index == digits.len() - 1 {
                // last so first to max
                let second_max = digits[..max_index].iter().max().unwrap();
                second_max * 10 + max
            } else {
                let second_max = digits[max_index + 1..].iter().rev().max().unwrap();
                max * 10 + second_max
            }
        })
        .sum::<u32>();

    println!("{joltage}");

    Ok(())
}

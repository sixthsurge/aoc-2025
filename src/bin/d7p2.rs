use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input/d7.txt")?
        .lines()
        .map(str::to_string)
        .collect_vec();

    // number of beams in each position
    let mut beams = vec![0i64; lines[0].len()];

    // initial beam
    beams[lines[0].chars().position(|c| c == 'S').unwrap()] = 1;

    for line in lines[1..].iter() {
        let reached_by_beam_old = beams.clone();

        for splitter_position in
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
        {
            beams[splitter_position] = 0;
            if splitter_position != 0 {
                beams[splitter_position - 1] += reached_by_beam_old[splitter_position];
            }
            if splitter_position != beams.len() - 1 {
                beams[splitter_position + 1] += reached_by_beam_old[splitter_position];
            }
        }
    }
    println!("{}", beams.iter().sum::<i64>());

    Ok(())
}

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input/d7.txt")?
        .lines()
        .map(str::to_string)
        .collect_vec();

    let mut beams = vec![false; lines[0].len()];

    // first line
    beams[lines[0].chars().position(|c| c == 'S').unwrap()] = true;

    let mut beams_old;
    let mut splits = 0;
    for line in lines[1..].iter() {
        beams_old = beams.clone();

        for (i, (beam, cell)) in beams_old.iter().copied().zip(line.chars()).enumerate() {
            if cell == '^' && beam {
                beams[i] = false;
                if i != 0 {
                    beams[i - 1] = true;
                }
                if i != beams.len() - 1 {
                    beams[i + 1] = true;
                }
                splits += 1;
            }
        }
    }
    println!("{splits}");

    Ok(())
}

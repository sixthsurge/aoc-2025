fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum = std::fs::read_to_string("input/d2p1.txt")?
        .split(",")
        .map(|min_max_str| {
            let mut siter = min_max_str.split("-");
            let mut c = 0;
            let f = siter.next().unwrap().trim();
            let s = siter.next().unwrap().trim();

            println!("{}", s);
            let min = f.parse::<usize>().unwrap_or_else(|_| {
                print!("{}", f);
                panic!()
            });
            let max = s.parse::<usize>().unwrap_or_else(|_| {
                print!("{}", s);
                panic!()
            });

            for b in min..=max {
                let bs = b.to_string();
                if bs.len() % 2 == 1 {
                    continue;
                }

                let n = bs.len() / 2;

                let one = &bs[..n];
                let two = &bs[n..bs.len()];
                if one == two {
                    c += b;
                }
            }
            c
        })
        .sum::<usize>();

    println!("{}", sum);
    Ok(())
}

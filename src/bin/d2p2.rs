fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum = std::fs::read_to_string("input/d2p1.txt")?
        .split(",")
        .map(|min_max_str| {
            let mut siter = min_max_str.split("-");
            let mut c = 0;
            let f = siter.next().unwrap().trim();
            let s = siter.next().unwrap().trim();

            let min = f.parse::<usize>().unwrap_or_else(|_| {
                print!("{}", f);
                panic!()
            });
            let max = s.parse::<usize>().unwrap_or_else(|_| {
                print!("{}", s);
                panic!()
            });

            for b in min..=max {
                let mut invalid = false;
                let bs = b.to_string();
                for group_count in 2..=bs.len() {
                    let group_length = bs.len() / group_count;
                    if bs.len() % group_count != 0 {
                        continue;
                    }

                    let mut i =
                        (0..group_count).map(|s| &bs[(group_length * s)..(group_length * (s + 1))]);
                    let c = i.clone().collect::<Vec<_>>();
                    let first = i.next().unwrap();
                    let invalids = i.all(|e| e == first);
                    if invalids {
                        invalid = true;
                        break;
                    }
                }
                if invalid {
                    c += b;
                }
            }
            c
        })
        .sum::<usize>();

    println!("{}", sum);
    Ok(())
}

use std::ops::RangeInclusive;

fn ranges_overlap(a: RangeInclusive<u64>, b: RangeInclusive<u64>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

// given two overlapping ranges, returns a set of non-overlapping ranges covering the same values
fn ranges_disjoint(a: RangeInclusive<u64>, b: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
    if a == b {
        return vec![a];
    }

    // 4 cases:
    //   - a starts and ends before b
    //     XXXXXXX
    //       XXXXXXX
    //     we return
    //     XXXXXXX
    //            XX
    //
    //   - b starts and ends before a
    //        XXXXXX
    //      XXXXXX
    //      we return
    //        XXXXXX
    //      XX
    //
    //   - a contains b => just return a
    //
    //   - b contains a => just return b

    if a.end() < b.end() {
        if a.start() < b.start() {
            // a before b
            vec![a.clone(), *a.end()+1..=*b.end()]
        } else {
            // b contains a
            vec![b]
        }
    } else {
        if b.start() < a.start() {
            // b before a
            vec![a.clone(), *b.start()..=*a.start()-1]
        } else {
            // a contains b
            vec![a]
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ranges = std::fs::read_to_string("input/d5.txt")?.lines().take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (min_str, max_str) = line.trim().split_once("-").unwrap();
            let min_val: u64 = min_str.parse().unwrap();
            let max_val: u64 = max_str.parse().unwrap();

            min_val..=max_val
        })
        .collect::<Vec<_>>();

    loop {
        let mut overlapping = None;
        'outer: for (i, range) in ranges.iter().enumerate() {
            for (j, other) in ranges.iter().enumerate() {
                if j == i {
                    continue;
                }
                if ranges_overlap(range.clone(), other.clone()) {
                    overlapping = Some((i, j));
                    break 'outer;
                }
            }
        }

        let Some((i, j)) = overlapping else { break; };

        let ri = ranges[i].clone();
        let rj = ranges[j].clone();

        ranges.remove(i);
        let j = if j > i { j - 1 } else { j }; // shift down after i
        ranges.remove(j);

        ranges.extend(ranges_disjoint(ri, rj).into_iter());
    }

    let count: u64 = ranges.into_iter().map(|range| range.end() - range.start() + 1).sum();
    println!("{count}");

    Ok(())
}


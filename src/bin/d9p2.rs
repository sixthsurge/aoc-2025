use itertools::Itertools;

struct Shape {
    h_arcs: Vec<(i64, i64, i64)>,
    v_arcs: Vec<(i64, i64, i64)>, // x, y1, y2
}

impl Shape {
    fn point_inside(&self, px: i64, py: i64) -> bool {
        // Point on h arc
        for &(ay, ax1, ax2) in self.h_arcs.iter() {
            let mix = i64::min(ax1, ax2);
            let max = i64::max(ax1, ax2);
            if (mix..=max).contains(&px) && py == ay {
                return true;
            }
        }

        // Sunday test

        let mut wn = 0;
        for &(ax, ay1, ay2) in self.v_arcs.iter() {
            // intersects if py between ys and px > ax
            let miy = i64::min(ay1, ay2);
            let may = i64::max(ay1, ay2);
            if (miy..may).contains(&py) {
                if px > ax {
                    if ay2 > ay1 {
                        wn += 1;
                    } else {
                        wn -= 1;
                    }
                } else if px == ax {
                    // point on the arc
                    return true;
                }
            }
        }
        wn != 0
    }

    fn arc_inside(&self, (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> bool {
        let (sx, sy) = ((x2 - x1).signum(), (y2 - y1).signum());
        assert!((sx == 0) || (sy == 0));

        // points to check: arc edges and points around crossing arcs
        if !self.point_inside(x1, y1) {
            return false;
        }
        if !self.point_inside(x2, y2) {
            return false;
        }
        if sy == 0 {
            // horizontal arc crosses vertical arcs
            for &(ax, _, _) in self.v_arcs.iter() {
                let mix = i64::min(x1, x2);
                let max = i64::max(x1, x2);
                if (mix..=max).contains(&ax) {
                    if !self.point_inside(ax, y1) {
                        return false;
                    }
                }
            }
        } else if sx == 0 {
            // vertical arc crosses horizontal arcs
            for &(ay, _, _) in self.h_arcs.iter() {
                let miy = i64::min(y1, y2);
                let may = i64::max(y1, y2);
                if (miy..=may).contains(&ay) {
                    if !self.point_inside(x1, ay) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn rect_inside(&self, (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> bool {
        // rect is inside if all arcs making rect are inside
        self.arc_inside((x1, y1), (x1, y2))
            && self.arc_inside((x2, y1), (x2, y2))
            && self.arc_inside((x1, y1), (x2, y1))
            && self.arc_inside((x1, y2), (x2, y2))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corners = std::fs::read_to_string("input/d9.txt")?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect_vec();

    let mut arcs = corners
        .iter()
        .copied()
        .zip(corners.iter().skip(1).copied())
        .collect_vec();
    arcs.push((
        corners.last().unwrap().clone(),
        corners.first().unwrap().clone(),
    ));
    assert!(arcs.first().unwrap().0 == arcs.last().unwrap().1);

    let h_arcs = arcs
        .iter()
        .filter_map(
            |&((x1, y1), (x2, y2))| {
                if y1 == y2 {
                    Some((y1, x1, x2))
                } else {
                    None
                }
            },
        )
        .collect_vec();

    let v_arcs = arcs
        .iter()
        .filter_map(
            |&((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    Some((x1, y1, y2))
                } else {
                    None
                }
            },
        )
        .collect_vec();

    let shape = Shape { h_arcs, v_arcs };

    let mut rects = itertools::iproduct!(0..corners.len(), 0..corners.len())
        .filter(|&(i1, i2)| {
            let c1 = corners[i1];
            let c2 = corners[i2];
            if c1.0 != c2.0 {
                c1.0 < c2.0
            } else {
                c1.1 < c2.1
            }
        })
        .collect_vec();
    rects.sort_unstable_by_key(|&(i1, i2)| {
        let c1 = corners[i1];
        let c2 = corners[i2];
        -(((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1))
    });

    let rects = rects;

    let mut k1 = None;
    let mut k2 = None;
    for (i, &(i1, i2)) in rects.iter().enumerate() {
        println!("{}/{}", i, rects.len());
        let c1 = corners[i1];
        let c2 = corners[i2];
        if shape.rect_inside(c1, c2) {
            let a = ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1);
            println!("{a}");
            k1 = Some(c1);
            k2 = Some(c2);
            break;
        }
    }

    /*
    // debug draw shape
    for i in 0..20 {
        for j in 0..20 {
            if Some((j, i)) == k1 {
                print!("O");
            } else if Some((j, i)) == k2 {
                print!("O");
            } else if shape.point_inside(j, i) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    */
    Ok(())
}

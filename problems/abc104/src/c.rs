use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        d: usize,
        g: i64,
        pcd: [(i64, i64); d]
    };
    let mut min = std::i64::MAX;
    for p in (0..d).permutations(d) {
        let mut gg = g;
        let mut total = 0;
        // println!("{:?}", p);
        for i in 0..d {
            if pcd[p[i]].0 * (p[i] as i64 +1) * 100 >= gg {
                total += (gg + ((p[i] as i64 +1) * 100) -1) / ((p[i] as i64 +1) * 100);
                gg = 0;
            } else {
                gg -= pcd[p[i]].0 * (p[i] as i64 +1) * 100 + pcd[p[i]].1;
                total += pcd[p[i]].0;
            }
            // println!("now {} total {} remain {}", p[i], total, gg);
            if gg <= 0 {
                break;
            }
        }
        if total < min {
            min = total;
        }
    }
    println!("{}", min);
}
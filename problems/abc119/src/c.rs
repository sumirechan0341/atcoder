use proconio::{input, marker::Chars};
use itertools::Itertools;
pub fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
        c: i32,
        ln: [i32; n]
    };
    let mut min = std::i32::MAX;
    // つなげる、伸ばす、縮めるは可換
    for i in 0..4_i32.pow(n as u32) {
        let mut ii = i;
        // つなげる方を全探索する
        // 使わないパターンもあるので4のn乗
        let mut total = vec![0; 3];
        let mut cost = 0;
        let mut remain = vec![];
        for j in 0..n {
            if ii % 4 == 0 {
                total[0] += ln[j];
                cost += 10;
            } else if ii % 4 == 1 {
                total[1] += ln[j];
                cost += 10;
            } else if ii % 4 == 2 {
                total[2] += ln[j];
                cost += 10;
            } else {
                remain.push(ln[j]);
            }
            ii /= 4;
        }
        if total[0] == 0 || total[1] == 0 || total[2] == 0 {
            continue;
        } 
        for c in (0..3).permutations(3) {
            let local = (a - total[c[0]]).abs() + (b - total[c[1]]).abs() + (b - total[c[2]]).abs();
            if local + cost < min {
                min = local + cost;
            }
        }
    }
    println!("{}", min);
}
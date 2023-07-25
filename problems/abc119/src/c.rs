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
        // つなげ方を全探索する
        // 使わないパターンもあるので4のn乗
        let mut total = vec![0; 3];
        let mut cost = 0;
        for j in 0..n {
            if ii % 4 == 0 {
                if total[0] != 0 {
                    cost += 10;
                }
                total[0] += ln[j];
            } else if ii % 4 == 1 {
                if total[1] != 0 {
                    cost += 10;
                }
                total[1] += ln[j];
            } else if ii % 4 == 2 {
                if total[2] != 0 {
                    cost += 10;
                }
                total[2] += ln[j];
            } else {
            }
            ii /= 4;
        }
        if total[0] == 0 || total[1] == 0 || total[2] == 0 {
            continue;
        } 
        for comb in (0..3).permutations(3) {
            let local = (a - total[comb[0]]).abs() + (b - total[comb[1]]).abs() + (c - total[comb[2]]).abs();
            if local + cost < min {
                min = local + cost;
            }
        }
    }
    println!("{}", min);
}
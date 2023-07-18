use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        psm: [(usize, String); m]
    };
    let mut ac = vec![false; n];
    let mut penalty = vec![0; n];
    let mut count = 0;
    for (p, s) in psm {
        if s == "AC" {
            if ac[p-1] {
                continue;
            }
            ac[p-1] = true;
            count += penalty[p-1];
        } else {
            penalty[p-1] += 1;
        }
    }
    println!("{} {}", ac.iter().filter(|&x| *x).count(), count);
}
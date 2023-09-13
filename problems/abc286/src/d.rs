use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: usize,
        abn: [(usize, usize); n]
    };
    let mut reachable = vec![false; x+1];
    reachable[0] = true;
    for &(a, b) in &abn {
        for i in (0..=x).rev() {
            for j in 1..=b {
                if i+j*a <= x && reachable[i] {
                    reachable[i+j*a] = true;
                }
            }
        }
    }
    println!("{}", if reachable[x] { "Yes" } else { "No" });
}
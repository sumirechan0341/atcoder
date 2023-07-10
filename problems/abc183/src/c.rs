use proconio::{input, marker::Chars};
use itertools::Itertools;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: i32,
        tnn: [[i32; n]; n]
    };
    let mut count = 0;
    for c in (1..n).permutations(n-1) {
        let mut t = tnn[0][c[0]];
        for i in 0..c.len()-1 {
            t += tnn[c[i]][c[i+1]];
        }
        t += tnn[c[n-2]][0];
        if t == k {
            count += 1;
        }
    }
    println!("{}", count);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        t: i64,
        tn: [i64; n]
    };
    let mut total = t*n as i64;
    for i in 1..n {
        total -= (t - (tn[i] - tn[i-1])).max(0);
    }
    println!("{}", total);
}
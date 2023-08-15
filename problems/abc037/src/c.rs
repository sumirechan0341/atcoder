use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        an: [i64; n]
    };
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i] + an[i];
    }
    let mut total = 0;
    for i in 0..n-k+1 {
        total += sn[i+k] - sn[i];
    }
    println!("{}", total);
}
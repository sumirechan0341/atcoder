use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [u64; n],
        x: u64
    };
    let mut s = vec![0; n];
    s[0] = an[0];
    for i in 1..n {
        s[i] = s[i-1] + an[i];
    }
    let cycle = s[n-1];
    println!("{}", (x/cycle)*n as u64 + if (x%cycle) == 0 { 1 } else {s.iter().take_while(|&y| *y <= x%cycle).count() as u64 + 1});
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        q: usize,
        lrq: [(usize, usize); q]
    };
    // 解説AC
    let mut imos = vec![0; n+1];
    for (l, r) in lrq {
        imos[l-1] = imos[l-1]^1;
        imos[r] = imos[r]^1
    }
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i]^imos[i];
    }
    for i in 1..=n {
        print!("{}", sn[i]);
    }
    println!("{}", "");
}
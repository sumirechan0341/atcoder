use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n]
    };
    let mut sn = vec![0; n + 1];
    for i in 0..n {
        sn[i + 1] = sn[i] + an[i];
    }
    let last = sn[n];
    sn.sort();
    println!("{:?}", if sn[0] >= 0 { last } else { last - sn[0] });
}

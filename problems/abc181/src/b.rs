use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        abn: [(i64, i64); n]
    };
    let mut sum = 0_i64;
    for ab in abn {
        sum += (ab.0..ab.1+1).sum::<i64>();
    }
    println!("{}", sum);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut min = std::i64::MAX;
    for i in -100..=100 {
        let mut local = 0;
        for j in 0..n {
            local += (i-an[j])*(i-an[j]);
        }
        if min > local {
            min = local;
        }
    }
    println!("{}", min);
}
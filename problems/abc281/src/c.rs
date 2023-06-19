use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut t: u64,
        an: [u64; n]
    };
    let cycle = an.iter().sum::<u64>();
    t %= cycle;

    for i in 0..n {
        if t <= an[i] {
            println!("{} {}", i+1, t);
            return;
        } else {
            t -= an[i];
        }
    }
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        xn: [i32; n]
    };
    let mut min = std::i32::MAX;
    for p in 0..=100 {
        let cost = xn.iter().map(|x| (x-p).pow(2)).sum();
        if cost < min {
            min = cost;
        }
    }
    println!("{}", min);
}
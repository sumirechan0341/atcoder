use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut prev_max = 0;
    let mut total = 0;
    for i in 0..n {
        if an[i] > prev_max {
            prev_max = an[i];
        } else {
            total += prev_max - an[i];
        }
    }
    println!("{}", total);
}
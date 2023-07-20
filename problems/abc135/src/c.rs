use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut an: [i64; n+1],
        bn: [i64; n]
    };
    let mut total = 0;
    for i in 0..n {
        if bn[i] > an[i] {
            total += an[i] + (bn[i] - an[i]).min(an[i+1]);
            an[i+1] -= (bn[i] - an[i]).min(an[i+1]);
        } else {
            total += bn[i];
        }
    }
        
    println!("{}", total);
}
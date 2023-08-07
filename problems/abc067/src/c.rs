use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut ls = vec![0; n-1];
    let mut rs = vec![0; n-1];
    ls[0] = an[0];
    rs[0] = an[n-1];
    for i in 1..n-1 {
        ls[i] = ls[i-1] + an[i];
        rs[i] = rs[i-1] + an[n-i-1];
    }
    
    let mut min = std::i64::MAX;

    for i in 0..n-1 {
        if (ls[i]-rs[n-i-2]).abs() < min {
            min = (ls[i]-rs[n-i-2]).abs();
        }
    }
    println!("{}", min);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        mut an: [i32; n]        
    };
    an.sort();
    an.reverse();
    let total = an.iter().sum::<i32>();
    println!("{}", if 4 * (m as i32) * an[m-1] >= total { "Yes" } else { "No" });
}
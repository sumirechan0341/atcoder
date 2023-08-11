use proconio::{input, marker::Chars};
use num::Integer;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        tan: [(i128, i128); n]
    };
    let mut t = 1;
    let mut a = 1;
    for i in 0..n {
        let mut not_satisfy = 0;
        let mut satisfy = 10_i128.pow(18);
        while satisfy-not_satisfy > 1 {
            let median = (satisfy + not_satisfy) / 2;
            let new_t = median*tan[i].0;
            let new_a = median*tan[i].1;
            if new_t >= t && new_a >= a {
                satisfy = median;
            } else {
                not_satisfy = median;
            }
        }
        t = tan[i].0*satisfy;
        a = tan[i].1*satisfy;
    }
    println!("{}", t+a);
}
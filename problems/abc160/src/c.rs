use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: i32,
        n: usize,
        an: [i32; n]
    };
    let mut max_diff = 0;
    for i in 0..n {
        if i == n-1 {
            if max_diff < k - an[i] + an[0] {
                max_diff = k - an[i] + an[0];
            }
        } else {
            if max_diff < (an[i] - an[i+1]).abs() {
                max_diff = (an[i] - an[i+1]).abs();
            }
        }
    }
    println!("{}", k-max_diff);
}
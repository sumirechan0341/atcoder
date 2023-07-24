use proconio::{input, marker::Chars};
use num::Integer;
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut from_left = vec![0; n+1];
    let mut from_right = vec![0; n+1];
    from_left[0] = 0;
    from_right[0] = 0;
    for i in 0..n {
        from_left[i+1] = from_left[i].gcd(&an[i]);
        from_right[i+1] = from_right[i].gcd(&an[n-i-1]);
    }
    let mut max = 0;
    for i in 0..n {
        let local = from_left[i].gcd(&from_right[n-i-1]);
        if local > max {
            max = local;
        }
    }
    println!("{}", max);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize
    };
    let mut total = 0.0;
    let mut nums = vec![0; n+1];
    for i in 1..=n {
        let mut ii = i;
        while k > ii {
            nums[i] += 1;
            ii *= 2;
        }
    }
    for i in 1..n+1 {
        total += 1.0 / (2.0_f64.powf(nums[i] as f64) * n as f64);
    }
    println!("{}", total);
}
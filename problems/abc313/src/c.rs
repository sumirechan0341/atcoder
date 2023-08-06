use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut an: [i64; n]
    };
    an.sort();
    an.reverse();
    let total = an.iter().sum::<i64>();
    let mean = total / n as i64;
    let remain = total % n as i64;
    let mut ans = 0;
    for i in 0..n {
        if i < remain as usize {
            ans += (mean+1-an[i]).abs();
        } else {
            ans += (mean-an[i]).abs();
        }
    }
    println!("{}", ans/2);
}
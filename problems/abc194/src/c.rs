use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i64; n]
    };
    let mut s = vec![0; n];
    s[n-1] = an[n-1];
    for i in (2..n).rev() {
        s[i-1] = s[i] + an[i-1];
    }
    let square = an.iter().map(|x| x*x).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        ans += square[i] * (n-1) as i64;
    }
    for i in 1..n {
        ans -= s[i] * an[i-1] * 2
    }
    println!("{}", ans);
}
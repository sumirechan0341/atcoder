use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        an: [i64; n]
    };
    let mut s1 = vec![0; n+1];
    let mut s2 = vec![0; n+1];
    s1[n-1] = an[n-1];
    for i in (1..n).rev() {
        s1[i-1] = s1[i] + an[i-1];
    }
    s2[n-1] = s1[n-1];
    for i in (1..n).rev() {
        s2[i-1] = s2[i] + s1[i-1];
    }
    let mut ans = std::i64::MIN;
    for i in (0..=n-m).rev() {
        let a = s2[i] - s2[i+m] - m as i64*s1[i+m];
        if a > ans {
            ans = a;
        }
    }

    println!("{}", ans);
}


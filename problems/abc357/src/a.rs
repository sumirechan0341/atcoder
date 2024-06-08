use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        hn: [usize; n]
    };
    let mut s = 0;
    let mut ans = 0;
    for i in 0..n {
        if s + hn[i] > m {
            break;
        }
        s += hn[i];
        ans += 1;
    }
    println!("{}", ans);
}

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        tq: [usize; q]
    };
    let mut ts = vec![true; n + 1];
    for t in tq {
        ts[t] = !ts[t];
    }
    let mut ans = 0;
    for i in 1..n + 1 {
        if ts[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        cn: [f64; n]
    };
    // 解説AC
    let mut cnt = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if cn[i] as i64 % cn[j] as i64 == 0 {
                cnt[i] += 1;
            }
        }
    }
    let mut ans = 0.0;
    for &c in &cnt {
        if c%2 == 1 {
            ans += 1.0/2.0;
        } else {
            ans += (c+2) as f64 / (2*c+2) as f64;
        }
    }
    println!("{}", ans);
}
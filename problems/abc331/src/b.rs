use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64,
        s: i64,
        m: i64,
        l: i64
    };
    let mut ans = std::i64::MAX;
    for i in 0..=n {
        for j in 0..=n {
            ans = ans.min((i + 5) / 6 * s + (j + 7) / 8 * m + (n - i - j + 11) / 12 * l)
        }
    }
    println!("{}", ans);
}

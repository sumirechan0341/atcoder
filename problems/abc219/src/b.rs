use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars
    };
    let mut ans = String::from("");
    let sv = vec![s1, s2, s3];
    for c in t {
        ans += &sv[(c.to_digit(10).unwrap() - 1) as usize];
    }
    println!("{}", ans);
}
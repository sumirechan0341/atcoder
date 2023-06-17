use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        s: Chars
    };
    let mut ans = String::new();
    for i in 0..n {
        ans += &s[i].to_string();
        ans += &s[i].to_string();
    }
    println!("{}", ans);
}
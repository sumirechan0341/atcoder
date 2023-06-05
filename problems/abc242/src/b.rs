use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut s: Chars
    };
    let mut ans = String::new();
    let mut search = s.clone();
    search.sort();
    while !search.is_empty() {
        ans += &search[0].to_string();
        search.remove(0);
    }
    println!("{}", ans);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars,
        t: Chars
    };
    for i in 0..s.len() {
        let news = [&s[i..], &s[0..i]].concat();
        if news == t {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
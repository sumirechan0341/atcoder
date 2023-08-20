use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let ans = &mut String::new();
    for c in s {
        if c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' {
            *ans += &c.to_string();
        }
    }
    println!("{}", ans);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32
    };
    println!("{}", s(n));
}
fn s(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    } else {
        return s(n-1) + &format!(" {} ", &n.to_string()) + &s(n-1);
    }
}
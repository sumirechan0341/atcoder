use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    for i in (0..s.len()).step_by(2) {
        print!("{}", s[i]);
    }
    println!("{}", "");
}
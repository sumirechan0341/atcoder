use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: String
    };
    println!("{}", s.replace("HAGIYA", "HAGIXILE"));
}
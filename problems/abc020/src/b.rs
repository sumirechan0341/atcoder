use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        mut a: Chars,
        b: Chars
    };
    a.extend(b);
    println!("{}", a.iter().collect::<String>().parse::<i32>().unwrap()*2);
}
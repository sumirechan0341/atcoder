use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: i32,
        b: i32
    };
    let mut s = vec![0];
    for i in 1..1000 {
        s.push(i as i32 + s[i-1]);
    }
    for i in 1.. {
        if b - a == s[i] - s[i-1] {
            println!("{}", s[i-1] - a);
            return;
        }
    }
}
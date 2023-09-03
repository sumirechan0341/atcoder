use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut s: Chars,
        t: Chars
    };
    for i in 0..s.len() {
        if s == t {
            println!("{}", i);
            return;
        }
        let c = s.pop().unwrap();
        s.reverse();
        s.push(c);
        s.reverse();
    }
    println!("{}", -1);
}
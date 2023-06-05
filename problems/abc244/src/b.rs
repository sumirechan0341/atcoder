use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        _n: usize,
        t: Chars
    };
    let mut p = (0, 0);
    let mut direction = (1, 0);
    for c in t {
        if c == 'R' {
            direction = (direction.1, -direction.0);
        } else {
            p = (p.0 + direction.0, p.1 + direction.1);
        }
    }
    println!("{} {}", p.0, p.1);
}
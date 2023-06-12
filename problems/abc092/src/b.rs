use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        d: i32,
        x: i32,
        an: [usize; n]
    };
    let mut total = 0;
    for a in an {
        for i in (1..=d).step_by(a) {
            total += 1;
        }
    }
    println!("{}", total + x);
}
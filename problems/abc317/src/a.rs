use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        h: i64,
        x: i64,
        pn: [i64; n]
    };
    for i in 0..n {
        if pn[i] + h >= x {
            println!("{}", i+1);
            return;
        }
    }
}
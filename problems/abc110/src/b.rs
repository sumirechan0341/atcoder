use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        x: i32,
        y: i32,
        mut xn: [i32; n],
        mut ym: [i32; m]
    };
    xn.push(x);
    xn.sort();
    ym.push(y);
    ym.sort();
    println!("{}", if xn[n] < ym[0] { "No War" } else { "War" });
}
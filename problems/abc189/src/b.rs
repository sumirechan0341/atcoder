use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        x: i32,
        vpn: [(i32, i32); n]
    };
    let mut a = 0;
    for i in 0..n {
        a += vpn[i].0 * vpn[i].1;
        if a > 100 * x {
            println!("{}", i+1);
            return;
        }
    }
    println!("{}", -1);
}
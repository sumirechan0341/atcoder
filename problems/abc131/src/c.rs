use proconio::{input, marker::Chars};
use num::Integer;
pub fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    };
    let nac = (a-1)/c;
    let nad = (a-1)/d;
    let nbc = b/c;
    let nbd = b/d;
    let nacd = (a-1)/c.lcm(&d);
    let nbcd = b/c.lcm(&d);
    println!("{}", (b-a+1) - ((nbc-nac) + (nbd-nad) - (nbcd-nacd)));
}
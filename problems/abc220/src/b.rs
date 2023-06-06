use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        k: u64,
        a: Chars,
        b: Chars
    };
    let mut ak = 0_u64;
    let mut bk = 0_u64;
    for i in 0..a.len() {
        ak += k.pow((a.len() - 1 - i) as u32)*(a[i].to_digit(10).unwrap() as u64);
    }
    for i in 0..b.len() {
        bk += k.pow((b.len() -1 - i) as u32)*(b[i].to_digit(10).unwrap() as u64);
    }
    println!("{}", ak * bk);
} 
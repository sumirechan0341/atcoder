use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a64: [u64; 64]
    };
    let mut ans: u64 = 0;
    for i in 0..64 {
        ans += a64[i] * 2_u64.pow(i as u32);
    }
    println!("{}", ans);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut ans = 0;
    for i in 0..s.len() {
        ans += (s[i] as u8 - 64) as u64 * 26_u64.pow((s.len()-1-i) as u32);
    }
    println!("{}", ans);
}
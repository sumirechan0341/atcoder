use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let diff = (s[0] as u8 + 26 - t[0] as u8) % 26;
    for i in 0..s.len() {
        if diff != (s[i] as u8 + 26 - t[i] as u8) % 26 {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
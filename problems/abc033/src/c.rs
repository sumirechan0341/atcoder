use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    let mut ans = 0;
    let mut acc = 1;
    for i in 0..s.len() {
        if s[i] == '+' {
            if acc != 0 {
                ans += 1;
            }
            acc = 1;
        } else if s[i].is_digit(10) {
            acc *= if s[i].to_digit(10).unwrap() == 0 {
                0
            } else {
                1
            };
        }
    }
    if acc != 0 {
        ans += 1;
    }
    println!("{}", ans);
}
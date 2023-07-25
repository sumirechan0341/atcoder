use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut cost1 = 0;
    let mut cost0 = 0;
    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i] == '0' {
                cost1 += 1;
            }
            if s[i] == '1' {
                cost0 += 1;
            }
        }
        if i % 2 == 1 {
            if s[i] == '1' {
                cost1 += 1;
            }
            if s[i] == '0' {
                cost0 += 1;
            }
        }
    }
    println!("{}", cost1.min(cost0));
}
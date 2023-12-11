use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: i64,
        s: Chars,
    };
    let mut ans = 0;
    let mut logo = 0;
    let mut empty = m;
    for i in 0..n {
        if s[i] == '0' {
            logo = ans;
            empty = m;
        } else if s[i] == '1' {
            if empty == 0 {
                if logo == 0 {
                    ans += 1;
                } else {
                    logo -= 1;
                }
            } else {
                empty -= 1;
            }
        } else {
            if logo == 0 {
                ans += 1;
            } else {
                logo -= 1;
            }
        }
    }
    println!("{}", ans);
}

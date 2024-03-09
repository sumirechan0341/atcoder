use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut ans = "".to_string();
    let mut mode = false;
    for i in 0..s.len() {
        if mode {
            if s[i] == '|' {
                mode = false;
            }
            continue;
        }
        if s[i] == '|' {
            mode = true;
        } else {
            ans += &s[i].to_string();
        }
    }
    println!("{}", ans);
}

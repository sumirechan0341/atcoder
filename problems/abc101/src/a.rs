use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        s: Chars        
    };
    let mut ans = 0;
    for c in s {
        if c == '+' {
            ans += 1;
        } else {
            ans -= 1;
        }
    }
    println!("{}", ans);
}
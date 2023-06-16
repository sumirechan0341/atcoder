use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    for i in 0..s.len() {
        if t[i] == '@' {
            match s[i] {
                'a' | 't' | 'c' | 'o' | 'd' | 'e' | 'r' | '@' => {
                    continue;
                }
                _ => {
                    println!("{}", "You will lose");
                    return;
                }
            }
        } else if s[i] == '@' {
            match t[i] {
                'a' | 't' | 'c' | 'o' | 'd' | 'e' | 'r' | '@' => {
                    continue;
                }
                _ => {
                    println!("{}", "You will lose");
                    return;
                }
            }
        } else {
            if t[i] != s[i] {
                println!("{}", "You will lose");
                return;
            }
        }
    }
    println!("{}", "You can win");
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    for i in 0..s.len() {
        if i == 0 {
            if s[i] != '<' {
                println!("No");
                return;
            }
        } else if i == s.len() - 1 {
            if s[i] != '>' {
                println!("{}", "No");
                return;
            }
        } else if s[i] != '=' {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}

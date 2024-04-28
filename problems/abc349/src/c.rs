use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let mut state = 0;
    for i in 0..s.len() {
        if s[i].to_ascii_uppercase() == t[state] {
            state += 1;
            if state == 3 {
                break;
            }
        }
        if t[state] == 'X' && state == 2 {
            state = 3;
            break;
        }
    }
    if state == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}

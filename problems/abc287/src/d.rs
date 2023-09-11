use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars,
        t: Chars
    };
    let mut left_head = 0;
    let mut right_head = t.len();
    for i in 0..t.len() {
        if s[i] == '?' || t[i] == '?' || s[i] == t[i] {
            left_head += 1;
        } else {
            break;
        }
    }
    for i in 0..t.len() {
        if s[s.len()-1-i] == '?' || t[t.len()-1-i] == '?' || s[s.len()-1-i] == t[t.len()-1-i] {
            right_head -= 1;
        } else {
            break;
        }
    }
    for i in 0..=t.len() {
        if right_head <= i && i <= left_head {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }

}
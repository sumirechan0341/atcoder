use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        s: Chars
    };
    println!("{}", if is_strong_parindrome(s) { "Yes" } else { "No" });
}

fn is_strong_parindrome(s: Vec<char>) -> bool {
    for i in 0..(s.len()/2) {
        if s[i] != s[s.len()-i-1] || s[i] != s[(s.len()-1)/2-1-i] || s[(s.len()+3)/2-1+i] != s[s.len()-i-1] {
            return false;
        }
    }
    return true;
}
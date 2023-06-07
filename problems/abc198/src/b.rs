use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: Chars
    };
    if n == vec!['0'] {
        println!("{}", "Yes");
        return;
    }
    while n[n.len()-1] == '0' {
        n.pop();
    }
    println!("{}", if is_parindrome(&n) { "Yes" } else { "No" });
}

fn is_parindrome(s: &Vec<char>) -> bool {
    for i in 0..s.len()/2 {
        if s[i] != s[s.len()-1-i] {
            return false;
        }
    }
    return true;
}
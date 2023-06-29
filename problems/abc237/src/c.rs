use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars
    };
    // 文字列の扱いについて
    // ある区間の文字を落としたいとき、popなどでひとつずつ落とすのは遅い
    if is_palindrome(&s) {
        println!("{}", "Yes");
        return;
    }
    let mut acount = 0;
    while s[s.len()-acount-1] == 'a' {
        acount += 1;
        if acount == s.len() {
            break;
        }
    }
    s = s[..s.len()-acount].to_vec();
    if s.is_empty() {
        println!("{}", "Yes");
        return;
    }
    let mut apref = 0;
    for i in 0..acount {
        if s[i] != 'a' {
            break;
        }
        apref += 1;
    }
    s = s[apref..].to_vec();

    if s.first().unwrap() == &'a' {
        println!("{}", "No");
        return;
    }
    if is_palindrome(&s) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
        
}

fn is_palindrome(s: &Vec<char>) -> bool {
    for i in 0..s.len()/2 {
        if s[i] != s[s.len()-1-i] {
            return false;
        }
    }
    return true;
}
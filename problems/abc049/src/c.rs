use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        s: Chars
    };
    println!("{}", if dfs(&s, 0) { "YES" } else { "NO" });
}

fn dfs(s: &Vec<char>, cursor: usize) -> bool {
    let word = &["dream","dreamer", "erase", "eraser"];
    for w in word {
        if cursor + w.len() > s.len() {
            continue;
        }
        let mut ok = true;
        for i in 0..w.len() {
            if s[cursor+i] != w.chars().nth(i).unwrap() {
                ok = false;
                break;
            }
        }
        if ok {
            if cursor+w.len() == s.len() {
                return true;
            } else {
                if dfs(s, cursor+w.len()) {
                    return true;
                }
            }
        }
    }
    return false;
}
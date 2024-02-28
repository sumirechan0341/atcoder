use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        cdq: [(char, char); q]
    };
    let mut goal = vec![0; 26];
    for i in 0..26 {
        goal[i as usize] = i;
    }
    for (c, d) in cdq {
        for i in 0..26 {
            if goal[i] == c as u8 - b'a' {
                goal[i] = d as u8 - b'a';
            }
        }
    }
    for i in 0..s.len() {
        print!("{}", (goal[(s[i] as u8 - b'a') as usize] + b'a') as char);
    }
    println!("{}", "");
}

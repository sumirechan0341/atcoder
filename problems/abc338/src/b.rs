use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut s: Chars
    };
    s.sort();
    let mut f = vec![0; 26];
    for i in 0..s.len() {
        f[(s[i] as u8 - 'a' as u8) as usize] += 1;
    }
    let max = f.iter().max().unwrap();
    for i in 0..s.len() {
        if f[(s[i] as u8 - 'a' as u8) as usize] == *max {
            println!("{}", s[i]);
            return;
        }
    }
}

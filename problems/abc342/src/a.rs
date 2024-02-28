use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let c = s[0];
    if s.iter().filter(|&&x| x == c).count() == 1 {
        println!("{}", 1);
        return;
    }
    for i in 0..s.len() {
        if s[i] != c {
            println!("{}", i + 1);
            return;
        }
    }
}

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    };
    let mut count = 0;
    for i in 0..n {
        if s[i] == 'o' && count < k {
            count += 1;
            print!("{}", 'o');
        } else {
            print!("{}", 'x');
        }
    }
    println!("{}", "");
}
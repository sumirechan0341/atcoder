use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    for inc in 1..n {
        let mut max_len = 0;
        for k in 0..n {
            if k + inc >= n || s[k] == s[k+inc] {
                break;
            } else {
                max_len += 1;
            }
        }
        println!("{}", max_len);
    }
}
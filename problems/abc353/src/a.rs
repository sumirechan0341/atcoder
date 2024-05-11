use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        hn: [i64; n]
    };
    for i in 1..n {
        if hn[0] < hn[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", -1);
}

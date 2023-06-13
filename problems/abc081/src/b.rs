use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    for i in 0.. {
        if (&an).into_iter().any(|x| x % 2_i32.pow(i) != 0) {
            println!("{}", i-1);
            return;
        }
    }
}
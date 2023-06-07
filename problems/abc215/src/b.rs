use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: u64
    };
    let mut k = 0;
    while n != 0 {
        n >>= 1;
        k += 1;
    }
    println!("{}", k - 1);
}
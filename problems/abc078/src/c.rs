use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        m: i32
    };
    println!("{}", ((n-m)*100+m*1900)*2_i32.pow(m as u32));
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: u64
    };
    println!("{}", format!("{:b}", k).replace("1", "2"));
}
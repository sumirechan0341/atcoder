use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    println!("{:02}:{:02}:{:02}", n/3600, (n - n/3600 * 3600) / 60 ,n%60);
}
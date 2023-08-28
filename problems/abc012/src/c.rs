use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i32
    };
    let mut total = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            total += i*j;
        }
    }
    let k = total-n;
    for i in 1..=9 {
        for j in 1..=9 {
            if i*j == k {
                println!("{} x {}", i, j);
            }
        }
    }
}
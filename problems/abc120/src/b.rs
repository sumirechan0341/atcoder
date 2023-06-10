use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        a: i32,
        b: i32,
        k: i32        
    };
    let mut count = 0;
    for i in (1..=a.min(b)).rev() {
        if a % i == 0 && b % i == 0 {
            count += 1;
        }
        if count == k {
            println!("{}", i);
            return;
        }
    }
}
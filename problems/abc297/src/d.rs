use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut a: i64,
        mut b: i64
    };
    let mut total = 0;
    loop {
        if a % b == 0 {
            total += a/b-1;
            break;
        } 
        if b % a == 0 {
            total += b/a-1;
            break;
        }
        if a > b {
            total += a/b;
            a %= b;
        } else {
            total += b/a;
            b %= a;
        }
    }
    println!("{}", total);
    
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    };
    if a%2 == 0 || b%2 == 0 || c%2 == 0 {
        println!("0");
        return;
    }
    println!("{}", (a*b).min(b*c).min(c*a));
    return;
}
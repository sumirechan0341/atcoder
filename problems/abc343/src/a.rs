use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64
    };
    if a == 0 && b == 0 {
        println!("{}", 1);
        return;
    } else {
        println!("{}", a.min(b));
    }
}

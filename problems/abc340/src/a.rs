use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        d: i64
    };
    for i in 0.. {
        let result = a + i * d;
        println!("{}", result);
        if result == b {
            break;
        }
    }
}

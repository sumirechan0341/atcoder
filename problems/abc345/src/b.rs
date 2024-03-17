use num::Integer;
use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        x: i64
    };
    println!("{}", x.div_ceil(&10));
}

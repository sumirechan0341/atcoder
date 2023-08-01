use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
        y: i32
    };
    let mut total = 0;
    if a + b > 2 * c {
        total += x.min(y) * 2 * c;
        total += (x - x.min(y)) * a.min(2*c) + (y - x.min(y)) * b.min(2*c);
    } else {
        total = a * x + b * y;
    }
    println!("{}", total);
}
use proconio::input;

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", if b % a == 0 { a + b } else { b - a });
}
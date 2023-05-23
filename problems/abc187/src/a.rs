use proconio::input;

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    println!("{}", s(a).max(s(b)));
}
fn s(n: i32) -> i32 {
    n / 100 + (n % 100) / 10 + (n % 10)
}
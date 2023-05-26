use proconio::input;

pub fn main() {
    input! {
        a: i32,
        _b: i32,
        _c: i32,
        _d: i32,
        e: i32,
        k: i32
    };
    println!("{}", if e - a <= k { "Yay!" } else { ":(" });
}
use proconio::input;

pub fn main() {
    input! {
        s: i32,
        w: i32
    };
    println!("{}", if s <= w { "unsafe" } else { "safe" });
}
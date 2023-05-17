use proconio::input;

pub fn main() {
    input !{
        a: i32,
        b: i32,
    };
    println!("{}", if b - a == 1 || b - a == 9 { "Yes" } else { "No" });
}
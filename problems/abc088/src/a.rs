use proconio::input;

pub fn main() {
    input !{
        n: i32,
        a: i32
    };
    println!("{}", if n % 500 <= a { "Yes" } else { "No" });
}
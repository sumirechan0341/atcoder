use proconio::input;

pub fn main() {
    input !{
        a: i32,
        b: i32
    };
    println!("{}", if a % 2 == 1 && b % 2 == 1 { "Odd" } else { "Even" });
}
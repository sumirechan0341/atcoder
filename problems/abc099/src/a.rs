use proconio::input;

pub fn main() {
    input !{
        n: i32
    };
    println!("{}", if n < 1000 { "ABC" } else { "ABD" });
}
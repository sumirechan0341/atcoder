use proconio::input;

pub fn main() {
    input !{
        r: i32
    };
    println!("{}", if r < 1200 { "ABC" } else if r < 2800 { "ARC" } else { "AGC" });
}
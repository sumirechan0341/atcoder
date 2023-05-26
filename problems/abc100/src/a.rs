use proconio::input;

pub fn main() {
    input !{
        a: i32,
        b: i32
    };
    println!("{}", if a.max(b) > 8 { ":(" } else { "Yay!" });
}
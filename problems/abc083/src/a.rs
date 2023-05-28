use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        a: i32,
        b: i32,
        c: i32,
        d: i32
    };
    let left = a + b;
    let right = c + d;
    println!("{}", match left - right { d if d < 0 => "Right", d if d > 0 => "Left", _ => "Balanced" });
}
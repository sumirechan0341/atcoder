use proconio::input;

pub fn main() {
    input !{
        x: i32,
        a: i32,
        b: i32
    };
    println!("{}", (x - a) % b);
}
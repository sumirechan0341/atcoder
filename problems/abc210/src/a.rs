use proconio::input;

pub fn main() {
    input !{
        n: i32,
        a: i32,
        x: i32,
        y: i32
    };
    println!("{}", n.min(a) * x + (n - a).max(0) * y);
}
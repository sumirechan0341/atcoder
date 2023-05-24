use proconio::input;

pub fn main() {
    input !{
        n: i32,
        r: i32
    };
    println!("{}", r + 100 * (10 - n.min(10)));
}
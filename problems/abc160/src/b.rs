use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        x: i32
    };
    let gohyaku = x / 500;
    let goen = (x - gohyaku * 500) / 5;
    println!("{}", gohyaku * 1000 + goen * 5);
}
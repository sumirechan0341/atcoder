use proconio::input;

pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    println!("{}", if x == y { x } else {3 - x - y});
}
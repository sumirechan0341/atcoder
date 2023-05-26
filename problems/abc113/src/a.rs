use proconio::input;

pub fn main() {
    input! {
        x: i32,
        y: i32
    };
    println!("{}", x + y / 2);
}
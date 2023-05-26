use proconio::input;

pub fn main() {
    input! {
        x: i32,
        a: i32
    };
    println!("{}", if x < a { 0 } else { 10 });
}
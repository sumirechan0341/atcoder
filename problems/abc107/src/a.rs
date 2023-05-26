use proconio::input;

pub fn main() {
    input! {
        n: i32,
        i: i32
    };
    println!("{}", n - i + 1);
}
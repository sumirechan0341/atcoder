use proconio::input;

pub fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    println!("{}", 21 - a - b - c);
}
use proconio::input;

pub fn main() {
    input! {
        n: usize,
        x: usize
    };
    println!("{}", "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth((x + n - 1) / n - 1).unwrap());
}
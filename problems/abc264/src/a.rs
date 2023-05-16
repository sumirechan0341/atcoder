use proconio::input;

pub fn main() {
    input! {
        l: usize,
        r: usize
    };
    println!("{}", "atcoder"[l-1..r].to_string());
}
use proconio::input;

pub fn main() {
    input! {
        k: usize
    }
    println!("{}", &"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..k]);
}
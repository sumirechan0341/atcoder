use proconio::input;

pub fn main() {
    input! {
        s: String
    }
    println!("{}", s.chars().nth(s.chars().count() / 2).unwrap())
}
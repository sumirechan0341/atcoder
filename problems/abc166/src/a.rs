use proconio::input;

pub fn main() {
    input! {
        s: String
    };
    println!("{}", if &s == "ABC" { "ARC" } else { "ABC" });
}
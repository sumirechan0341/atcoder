use proconio::input;

pub fn main() {
    input !{
        s: String
    };
    println!("{}", if &s == "AAA" || &s == "BBB" { "No" } else { "Yes" });
}
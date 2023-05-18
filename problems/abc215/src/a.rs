use proconio::input;

pub fn main() {
    input !{
        s: String
    };
    println!("{}", if s == "Hello,World!" { "AC" } else { "WA" });
}
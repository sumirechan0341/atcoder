use proconio::input;

pub fn main() {
    input! {
        s: String,
        t: String
    };
    println!("{}", t + &s);
}
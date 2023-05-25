use proconio::input;

pub fn main() {
    input! {
        a: i32,
        s: String
    };
    println!("{}", if a >= 3200 { s } else { "red".to_string() });
}
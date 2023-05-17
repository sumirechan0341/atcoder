use proconio::input;
pub fn main() {
    input! {
        n: String
    };
    println!("{}", &n[1..=2]);
}
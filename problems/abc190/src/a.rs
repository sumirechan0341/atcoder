use proconio::input;

pub fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32
    };
    println!("{}", if c == 0 { if a > b  { "Takahashi" } else { "Aoki" }} else { if b > a  { "Aoki" } else { "Takahashi" } });
}
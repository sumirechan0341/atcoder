use proconio::input;

pub fn main() {
    input! {
        n: i64
    };
    println!("{}", if n >= -2i64.pow(31) && n < 2i64.pow(31) { "Yes" } else { "No" } );
}
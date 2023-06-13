use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i32,
        b: i32
    };
    let bd = if b / 100 > 0 { 3 } else if b / 10 > 0 { 2 } else { 1 };
    let dict = (0..400).map(|i| i*i).collect::<Vec<_>>();
    println!("{}", if dict.contains(&(10_i32.pow(bd) * a + b)) { "Yes" } else { "No" });
}
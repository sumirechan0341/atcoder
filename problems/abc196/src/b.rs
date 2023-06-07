use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: Chars
    };
    let pos = x.iter().position(|c| c == &'.').unwrap_or(x.len());
    println!("{}", x[..pos].iter().collect::<String>());
}
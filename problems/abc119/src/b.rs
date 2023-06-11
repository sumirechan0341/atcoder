use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        xun: [(f32, String); n]
    };
    println!("{}", xun.into_iter().map(|(x, u)| if &u == "JPY" { x } else { 380000.0 * x }).sum::<f32>());
}
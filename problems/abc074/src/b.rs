use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        k: i32,
        xn: [i32; n]
    };
    let mut total = 0;
    for i in 0..n {
        total += 2 * ((xn[i]-k).abs().min(xn[i]).abs());
    }
    println!("{}", total);
}
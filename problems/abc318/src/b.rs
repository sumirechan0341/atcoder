use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        abcdn: [(i32, i32, i32, i32); n]
    };
    let mut total = 0;
    for i in 0..101 {
        for j in 0..101 {
            for &(a, b, c, d) in &abcdn {
                if a <= i && i < b && c <= j && j < d {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("{}", total);
}
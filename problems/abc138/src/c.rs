use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut vn: [i64; n]
    };
    vn.sort();
    let mut total = vn[0];
    for i in 1..n {
        total += vn[i]*2_i64.pow((i-1) as u32);
    }
    println!("{}", total as f64 / 2_i64.pow((n-1) as u32) as f64);
}
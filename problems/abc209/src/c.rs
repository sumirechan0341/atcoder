use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut cn: [i64; n]
    };
    cn.sort();
    let p = 1000000007;
    let mut total = 1;
    for i in 0..n {
        total *= (cn[i]-i as i64).max(0);
        total %= p;
    }
    println!("{}", total);
}
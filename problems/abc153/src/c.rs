use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        mut hn: [i64; n]
    };
    hn.sort();
    hn.reverse();
    let mut total = 0;
    for i in k..n {
        total += hn[i];
    }
    println!("{}", total);
}
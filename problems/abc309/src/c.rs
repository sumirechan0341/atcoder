use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: i64,
        mut abn: [(i64, i64); n]
    };
    abn.sort();
    let mut total = 0;
    for i in 0..n {
        total += abn[i].1;
    }
    if total <= k {
        println!("{}", 1);
        return;
    }
    for i in 0..n {
        total -= abn[i].1;
        if total <= k {
            println!("{}", abn[i].0+1);
            return;
        } 
    }
}
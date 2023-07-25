use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut m: i64,
        mut abn: [(i64, i64); n]
    };
    abn.sort();
    let mut total = 0;
    for i in 0..n {
        if abn[i].1 >= m {
            total += abn[i].0 * m;
            break;
        }
        total += abn[i].0 * abn[i].1;
        m -= abn[i].1;
    }
    println!("{}", total);
}
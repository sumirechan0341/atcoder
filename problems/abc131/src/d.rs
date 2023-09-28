use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut abn: [(i64, i64); n]
    };
    abn.sort_by_key(|x| x.1);
    let mut time = 0;
    for i in 0..n {
        if time + abn[i].0 > abn[i].1 {
            println!("{}", "No");
            return;
        }
        time += abn[i].0;
    }
    println!("{}", "Yes");
}
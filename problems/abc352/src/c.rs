use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut abn: [(i64, i64); n]
    };
    abn.sort_by(|&a, &b| (a.0 - a.1).cmp(&(b.0 - b.1)));
    abn.reverse();
    let mut ans = 0;
    for i in 0..n - 1 {
        ans += abn[i].0;
    }
    ans += abn[n - 1].1;
    println!("{}", ans);
}

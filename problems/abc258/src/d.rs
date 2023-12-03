use proconio::{input, marker::Chars};
pub fn main() {
    input! {
        n: usize,
        x: i64,
        abn: [(i64, i64); n]
    };
    let mut smin = vec![std::i64::MAX; n + 1];
    let mut sn = vec![0; n + 1];
    for i in 0..n {
        smin[i + 1] = smin[i].min(abn[i].1);
        sn[i + 1] = sn[i] + abn[i].0 + abn[i].1;
    }
    let mut ans = std::i64::MAX;

    for i in 0..n {
        ans = ans.min(sn[i + 1] + (x - i as i64 - 1) * smin[i + 1]);
    }
    println!("{}", ans);
}

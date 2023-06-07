use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        apxn: [(i32, i32, i32); n]
    };
    let mut ans = apxn.iter().map(|apx| ((apx.2 - apx.0).max(0), apx.1)).collect::<Vec<_>>();
    ans.sort_by_key(|apx| apx.1);
    for i in 0..n {
        if ans[i].0 > 0 {
            println!("{}", ans[i].1);
            return;
        }
    }
    println!("{}", -1);
}
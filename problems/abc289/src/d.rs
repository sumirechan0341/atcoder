use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n],
        m: usize,
        bm: [usize; m],
        x: usize
    };
    let mut mochi = vec![false; x+1];
    for &b in &bm {
        mochi[b] = true;
    }
    // BFSにする
    struct DFS<'a> { f: &'a dyn Fn(&DFS, usize) -> bool }
    let dfs = DFS {
        f: &|dfs, now: usize| -> bool {
            if now == 0 {
                return true;
            }
            let mut res = false;
            for &a in &an {
                if now < a || mochi[now - a] {
                    continue;
                } else {
                    res = res || (dfs.f)(dfs, now - a);
                }
            }
            return res;
        }
    };
    println!("{}", if (dfs.f)(&dfs, x) { "Yes" } else { "No" });
}
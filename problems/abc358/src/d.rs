use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        mut an: [usize; n],
        mut bm: [usize; m]
    };
    an.sort();
    bm.sort();
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..m {
        let j = an[cur..].partition_point(|&x| x < bm[i]);
        if j == an[cur..].len() {
            println!("{}", -1);
            return;
        } else {
            ans += an[cur..][j];
            cur += j + 1;
        }
    }
    println!("{}", ans);
}

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        p: i64,
        mut an: [i64; n],
        mut bm: [i64; m]
    };
    an.sort();
    bm.sort();
    let mut sn = vec![0; n+1];
    for i in 0..n {
        sn[i+1] = sn[i]+an[i];
    }
    let mut ans = 0;
    for &b in &bm {
        let mut idx = an.partition_point(|x| x+b < p);
        ans += ((n-idx) as i64)*p;
        ans += sn[idx]+(b*idx as i64);
    }
    println!("{}", ans);
}
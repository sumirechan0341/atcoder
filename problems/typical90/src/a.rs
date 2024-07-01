use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut an: [usize; n]
    };
    an.push(l);
    let mut ok = 0 as i64;
    let mut ng = 1e9 as i64;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid, &an, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn f(x: i64, an: &Vec<usize>, k: usize) -> bool {
    let mut cut = 0;
    let mut prev = 0;
    for i in 0..an.len() {
        if an[i] as i64 - prev >= x {
            cut += 1;
            prev = an[i] as i64;
        }
    }
    cut >= k + 1
}

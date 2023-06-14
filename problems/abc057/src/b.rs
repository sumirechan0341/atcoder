use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        abn: [(i32, i32); n],
        cdm: [(i32, i32); m]
    };
    let mut ans = vec![];
    for ab in abn {
        let mut min_index = 0;
        for i in 0..m {
            if md(ab, cdm[min_index]) > md(ab, cdm[i]) {
                min_index = i;
            }
        }
        ans.push(min_index);
    }
    for i in 0..n {
        println!("{}", ans[i]+1);
    }
}
fn md(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
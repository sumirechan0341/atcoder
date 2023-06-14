use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        tn: [i32; n],
        m: usize,
        pxm: [(i32, i32); m]
    };
    for i in 0..m {
        let mut total = 0;
        for j in 0..n {
            let target_p = pxm[i].0;
            total += if j as i32 == target_p-1 {
                pxm[i].1
            } else {
                tn[j]
            }
        }
        println!("{}", total);
    }
}
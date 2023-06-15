use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        q: usize,
        lrtq: [(usize, usize, i32); q]
    };

    let mut ans = vec![0; n];
    for (l, r, t) in lrtq {
        for a in &mut ans[l-1..=r-1] {
            *a = t;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
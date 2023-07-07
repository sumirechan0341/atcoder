use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tabq: [(i32, usize, usize); q]
    };
    // 文字列の分割
    // sliceのswap
    // スワップ
    let (mut left, mut right) = s.split_at_mut(n);
    for (t, a, b) in tabq {
        if t == 1 {
            if a > n {
                right.swap(a-n-1, b-n-1);
            } else if b > n {
                let tmp = left[a-1];
                left[a-1] = right[b-n-1];
                right[b-n-1] = tmp;
            } else {
                left.swap(a-1, b-1);
            }
        } else {
            let tmp = left;
            left = right;
            right = tmp;
        }
    }
    println!("{}", [left, right].concat().iter().collect::<String>());
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: i64,
        mut an: [i64; n]
    };
    an.sort();
    let mut left = 0;
    let mut right = 0;
    let mut max = 0;
    while left < n {
        while right < n && an[right]-an[left] < m {
            right += 1;
        }
        if max < right-left {
            max = right-left;
        }
        left += 1;
        right = right.max(left);
    }
    println!("{}", max);
}
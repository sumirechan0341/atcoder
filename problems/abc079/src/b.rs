use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let mut memo: Vec<i64> = vec![2, 1];
    for i in 2..=86 {
       memo.push(memo[i-2]+memo[i-1]);
    }
    println!("{}", memo[n]);
}
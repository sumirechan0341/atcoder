use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        k: i32,
        m: i32,
        an: [i32; n-1]
    };
    let total = an.iter().sum::<i32>();
    println!("{}", if (n as i32) * m - total <= k { ((n as i32) * m - total).max(0) } else { -1 });
} 
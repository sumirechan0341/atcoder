use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: i32
    };
    let mut ans = Vec::new();
    let mut count = 0;
    let mut bit = 1;
    while n > 0 {
        if (n & 1) == 1 {
            count += 1;
            ans.push((n & 1) * bit);
        }
        n >>= 1;
        bit <<= 1;
    }
    println!("{}", count);
    for a in ans {
        println!("{}", a);
    }
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: u64
    };
    let mut ans = vec![];
    while n != 0 {
        if n & 1 == 1 {
            ans.push('A');
            n -= 1;
        } else {
            ans.push('B');
            n = n >> 1
        }
    }
    ans.reverse();
    for a in ans {
        print!("{}", a);
    }
    println!("{}", "");
}
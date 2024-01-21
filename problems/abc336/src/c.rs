use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: usize,
    };
    if n == 1 {
        println!("{}", 0);
        return;
    }
    n -= 1;
    let dict = [0, 2, 4, 6, 8];
    let mut ans = vec![];
    while n != 0 {
        ans.push(n % 5);
        n /= 5;
    }
    ans.reverse();
    for i in 0..ans.len() {
        print!("{}", dict[ans[i]]);
    }
}

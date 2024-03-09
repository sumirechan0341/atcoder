use proconio::{input, marker::Chars};

pub fn main() {
    let mut ans = vec![];
    loop {
        input! {
            n: usize
        }
        ans.push(n);
        if n == 0 {
            break;
        }
    }
    ans.reverse();
    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}

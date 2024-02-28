use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let mut ans = vec![];
    for i in 0..2 * n + 1 {
        if i % 2 == 0 {
            ans.push("1");
        } else {
            ans.push("0");
        }
    }
    println!("{}", ans.join(""));
}

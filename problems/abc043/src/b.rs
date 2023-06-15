use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        s: Chars
    };
    let mut ans = vec![];
    for c in s {
        match c {
            '0' => {
                ans.push("0");
            },
            '1' => {
                ans.push("1");
            },
            _ => {
                ans.pop();
            },
        }
    }
    println!("{}", ans.join(""));
}
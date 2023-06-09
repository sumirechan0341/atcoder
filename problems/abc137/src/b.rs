use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        k: i32,
        x: i32
    };
    let mut ans = vec![];
    for i in 0..2*k-1 {
        ans.push((x-k+1+i).to_string());
    }
    println!("{}", ans.join(" "));
}
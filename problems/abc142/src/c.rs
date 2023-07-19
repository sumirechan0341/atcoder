use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut ans = vec!["".to_string(); n];
    for i in 0..n {
        ans[an[i]-1] = (i+1).to_string();
    }
    println!("{}", ans.join(" "));
}
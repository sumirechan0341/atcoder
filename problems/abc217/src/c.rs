use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [usize; n]
    };
    let mut ans = vec!["0".to_string(); n];
    for i in 0..n {
        ans[pn[i]-1] = (i+1).to_string();
    }
    println!("{}", ans.join(" "));
}
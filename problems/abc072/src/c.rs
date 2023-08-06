use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n]
    };
    let mut ans = vec![0; 100002];
    for i in 0..n {
        if an[i] == 0 {
            ans[an[i]] += 1;
            ans[an[i]+1] += 1;
        } else {
            ans[an[i]-1] += 1;
            ans[an[i]] += 1;
            ans[an[i]+1] += 1;
        }
    }
    println!("{}", ans.iter().max().unwrap());
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        s: Chars
    };
    let mut ws = vec![0; n+1];
    let mut es = vec![0; n+1];

    for i in 0..n {
        if s[i] == 'W' {
            ws[i+1] = ws[i]+1;
        } else {
            ws[i+1] = ws[i];
        }
        if s[n-1-i] == 'E' {
            es[i+1] = es[i]+1;
        } else {
            es[i+1] = es[i];
        }
    }
    let mut ans = std::i32::MAX;
    for i in 0..=n {
        if es[n-i] + ws[i] < ans {
            ans = es[n-i] + ws[i];
        }
    }
    println!("{}", ans);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n]
    };
    let mut open = vec![-1; n+1];
    let mut close = vec![-1; n+1];
    for i in 0..n {
        if open[an[i]] == -1 {
            open[an[i]] = i as i32;
        }
        if open[an[i]] > -1 {
            close[an[i]] = i as i32;
        }
    }

}
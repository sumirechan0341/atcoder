use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        hn: [i32; n]
    };
    for i in 0..n {
        if i == n-1 {
            println!("{}", hn[i]);
            return;
        }
        if hn[i] >= hn[i+1] {
            println!("{}", hn[i]);
            return;
        }
    }
}
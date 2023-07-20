use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        hn: [i32; n]
    };
    for i in 0..n-1 {
        if hn[i] > hn[i+1] + 1 {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
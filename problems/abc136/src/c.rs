use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut hn: [i32; n]
    };
    for i in (1..n).rev() {
        if hn[i] < hn[i-1] {
            if hn[i-1] - hn[i] == 1 {
                hn[i-1] -= 1;
            } else {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}
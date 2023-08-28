use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        k: usize,
        tkn: [[i32; k]; n]
    };
    for i in 0..k.pow(n as u32) {
        let mut ii = i;
        let mut total = 0;
        for j in 0..n {
            total ^= tkn[j][ii%k];
            ii /= k;
        }
        if total == 0 {
            println!("{}", "Found");
            return;
        }
    }
    println!("{}", "Nothing");
}
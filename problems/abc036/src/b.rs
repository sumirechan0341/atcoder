use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        snn: [Chars; n]
    };
    // 90度回転 検索用
    for i in 0..n {
        for j in 0..n {
            print!("{}", snn[n-j-1][i]); 
        }
        println!("{}", "");
    }
}
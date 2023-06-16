use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        an: [i32; n]
    };
    // 検索用 bit ビット bit全探索 ビット全探索
    let mut total = 0;
    for i in 0..n {
        let b = 2_i32.pow(i as u32);
        if (b & x) == b {
            total += an[i];
        }
    }
    println!("{}", total);
}
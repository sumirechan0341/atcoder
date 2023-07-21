use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        l: i64,
        r: i64
    };
    let new_l = l % 2019;
    let new_r = r % 2019;
    let mut min = std::i64::MAX;
    if new_l >= new_r || r - l >= 2019 {
        println!("{}", 0);
        return;
    }
    for i in new_l..new_r+1 {
        for j in i+1..new_r+1 {
            if i * j % 2019 < min {
                min = i * j % 2019;
            }
        }
    }
    println!("{}", min);
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32,
        y: i32
    };
    for i in 0..=y/10000 {
        let remain1 = y-i*10000;
        for j in 0..=remain1/5000 {
            let remain2 = remain1-j*5000;
            let k = remain2 / 1000;
            if i+j+k == n {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
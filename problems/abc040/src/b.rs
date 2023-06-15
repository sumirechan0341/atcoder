use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    let mut min = std::i32::MAX;
    for i in 1..=n {
        let mut p = 0;
        let mut q = 0;
        for j in 1.. {
            if j*j > i {
                break;
            }
            if i % j == 0 {
                p = j;
                q = i / j;
            }
        }
        let d = (p-q).abs();
        if d + (n-i) < min {
            min = d + (n-i);
        }
    }
    println!("{}", min);
}
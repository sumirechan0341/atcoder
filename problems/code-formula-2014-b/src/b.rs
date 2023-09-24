use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: Chars
    };
    let mut odd = 0;
    let mut even = 0;
    let mut status = true;
    for i in (0..n.len()).rev() {
        if status {
            odd += n[i].to_digit(10).unwrap();
        } else {
            even += n[i].to_digit(10).unwrap();
        }
        status = !status;
    }
    println!("{} {}", even, odd);
}
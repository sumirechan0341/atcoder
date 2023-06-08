use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: u64
    };
    let mut sum: u64 = 100;
    for i in 1.. {
        sum += sum / 100;
        if sum >= x {
            println!("{}", i);
            return;
        }
    }
}
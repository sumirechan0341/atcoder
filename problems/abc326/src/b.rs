use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i32
    };
    for i in n..=919 {
        if (i / 100) * ((i%100) / 10) == i % 10 {
            println!("{}", i);
            return;
        }
    }
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        an: [i32; n]
    };
    println!("{}", if an.iter().enumerate().map(|(i, x)| if i % 2 == 1 { x-1 } else { *x }).collect::<Vec<_>>().iter().sum::<i32>() <= x { "Yes" } else { "No" } );
}
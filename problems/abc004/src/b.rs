use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut chw: [[String; 4]; 4]
    };
    for i in (0..4).rev() {
        chw[i].reverse();
        println!("{}", chw[i].join(" "));
    }
}
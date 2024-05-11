use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize
    };
    for i in x.min(y)..=x.max(y) {
        if i == z {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}

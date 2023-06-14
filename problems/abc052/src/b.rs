use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut max = 0;
    let mut x = 0;
    for c in s {
        match c {
            'I' => {
                x += 1;
                if max < x {
                    max = x;
                }
            },
            _ => {
                x -= 1;
            }
        }
    }
    println!("{}", max);
}
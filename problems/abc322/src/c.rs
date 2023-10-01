use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m]
    };
    let mut cursor = 0;
    for i in 1..=n {
        while am[cursor] < i {
            cursor += 1;
        }
        println!("{}", am[cursor]-i);
    }
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        p: [i32; n-1]
    };

    let mut current = n;
    let mut count = 0;
    while true {
        if p[current-2] == 1 {
            println!("{}", count + 1);
            return;
        }
        count += 1;
        current = p[current-2] as usize;
    }
}
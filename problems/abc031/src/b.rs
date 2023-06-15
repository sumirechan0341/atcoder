use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        l: i32,
        h: i32,
        n: usize,
        an: [i32; n]
    };
    for a in an {
        if a < l {
            println!("{}", l-a);
        } else if a > h {
            println!("{}", -1);
        } else {
            println!("{}", 0);
        }
    }
}
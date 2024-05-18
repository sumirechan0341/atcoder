use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: usize,
    };
    let mut p = 1;
    for i in 1.. {
        if p + 2_usize.pow(i) > h {
            println!("{}", i + 1);
            break;
        }
        p += 2_usize.pow(i);
    }
}

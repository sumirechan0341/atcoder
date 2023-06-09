use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        pn: [usize; n]
    };
    let mut count = 0;
    for i in 0..n {
        if pn[i] != i+1 {
            count += 1;
        }
    }
    if count > 2 {
        println!("{}", "NO");
    } else {
        println!("{}", "YES");
    }
}
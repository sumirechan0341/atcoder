use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        dn: [(i32, i32); n]
    };
    let mut zoro = 0;
    for (d1, d2) in dn {
        if d1 == d2 {
            zoro += 1;
        } else {
            zoro = 0;
        }
        if zoro == 3 {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
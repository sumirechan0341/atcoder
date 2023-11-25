use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        an: [i32; n]
    };
    for i in 0..n {
        if an[i] <= l {
            println!("{}", l);
        } else if an[i] >= r {
            println!("{}", r);
        } else {
            println!("{}", an[i]);
        }
    }
}

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let v = an[0];
    for i in 0..n {
        if v != an[i] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}

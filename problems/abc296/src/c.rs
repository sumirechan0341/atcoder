use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        mut an: [i32; n]
    };
    an.sort();
    for a in &an {
        match an.binary_search(&(a+x)) {
            Ok(x) => { 
                println!("{}", "Yes");
                return;
            },
            Err(_) => {
                continue;
            }
        };
    }
    println!("{}", "No");
}
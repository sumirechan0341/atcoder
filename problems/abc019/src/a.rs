use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut abc: [i32; 3]
    };
    abc.sort();
    println!("{}", abc[1]);
}
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: usize,
        sa: [(String, i64); n]
    };
    let mut min_index = 0;
    for i in 0..n {
        if sa[min_index].1 > sa[i].1 {
            min_index = i;
        }
    }
    for i in 0..n {
        println!("{}", sa[(i+min_index)%n].0);
    }
}
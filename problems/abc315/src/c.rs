use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut fsn: [(i32, i32); n]
    };
    fsn.sort_by_key(|x| x.1);
    let max = fsn[n-1];
    for i in 0..n-1 {
        if max.0 == fsn[i].0 {
            fsn[i].1 = fsn[i].1/2;
        }
    }
    fsn.sort_by_key(|x| x.1);
    println!("{}", fsn[n-1].1+fsn[n-2].1);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        xys: [(i32, i32); 2]
    };
    println!("{}", (xys[0].0-xys[1].0).abs()+(xys[0].1-xys[1].1).abs()+1);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        x: i32
    };
    for i in 1..=x {
        if i*(i+1)/2 >= x {
            println!("{}", i);
            return;
        }
    }
}
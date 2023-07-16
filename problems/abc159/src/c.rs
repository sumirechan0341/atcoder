use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        l: f64
    };
    println!("{}", l*l*l / 27.0);
}
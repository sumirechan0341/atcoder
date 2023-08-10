use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        x: i64
    };
    if x%11 == 0 {
        println!("{}", x/11*2);
    } else if x % 11 <= 6 {
        println!("{}", x/11*2+1);
    } else {
        println!("{}", x/11*2+2);
    }
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        a: i32,
        b: i32
    };
    if b % 3 == 0 || b % 3 == 2 {
        if b - 1 == a {
            println!("{}", "Yes");
            return;
        } else {
            println!("{}", "No");
            return;
        }
    } else if b % 3 == 1 {
        println!("{}", "No");
    } 
}
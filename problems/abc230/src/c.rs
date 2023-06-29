use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: u64,
        a: i128,
        b: i128,
        p: i128,
        q: i128,
        r: i128,
        s: i128
    };
    for i in p..=q {
        for j in r..=s {
            if a-i == b-j || a-i == -(b-j) {
                print!("{}", '#');
            } else {
                print!("{}", '.');
            }
        }
        println!("{}", "");
    }
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        mut an: [i32; n]
    };
    an.sort();
    for i in 1..n {
        if an[i]-an[i-1] != 1 {
            println!("{}", an[i]-1);
            return;
        }
    }
}
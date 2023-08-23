use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n]
    };
    let mut count = 0;
    for i in 0..n {
        if an[an[i]-1] == i+1 {
            count += 1;
        }
    }
    println!("{}", count/2);
}
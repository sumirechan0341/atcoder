use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64
    };
    let mut count = 0;
    for i in 1.. {
        if n < 10_i64.pow(3*i) {
            break;
        }
        count += n-10_i64.pow(3*i)+1
    }
    println!("{}", count);
}
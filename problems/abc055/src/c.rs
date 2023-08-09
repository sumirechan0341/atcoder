use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64,
        m: i64
    };
    if n*2 >= m {
        println!("{}", m/2);
        return;
    }
    let remain = m-2*n;
    println!("{}", remain/4+n);
}
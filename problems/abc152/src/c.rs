use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        pn: [i32; n]
    };
    let mut min = pn[0];
    let mut count = 0;
    for i in 0..n {
        if pn[i] <= min {
            count += 1;
            min = pn[i];
        }
    }
    println!("{}", count);
}
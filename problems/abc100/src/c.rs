use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut count = 0;
    for i in 0..n {
        let mut m = an[i];
        while m > 0 {
            if m & 1 == 1 {
                break;
            } else {
                count += 1;
            }
            m = m >> 1;
        }
    }
    println!("{}", count);
}
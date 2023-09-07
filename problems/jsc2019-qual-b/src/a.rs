use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        m: i32,
        d: i32
    };
    let mut count = 0;
    for i in 1..=m {
        for j in 1..=d {
            if j/10 < 2 || j%10 < 2 {
                continue;
            }
            if i == j/10 * (j%10) {

                count += 1;
            }
        }
    }
    println!("{}", count);
}
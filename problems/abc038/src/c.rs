use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let mut total = 0;
    let mut count = 1_i64;
    for i in 0..n-1 {
        if an[i] < an[i+1] {
            count += 1;
        } else {
            total += count*(count+1)/2;
            count = 1;
        }
    }
    total += count*(count+1)/2;
    println!("{}", total);
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        x: i64,
        mut an: [i64; n]
    };
    let mut total = 0;
    for i in 0..n-1 {
        if an[i] + an[i+1] > x {
            let exceed = an[i] + an[i+1] - x;
            if an[i+1] >= exceed {
                an[i+1] -= exceed;
            } else {
                an[i+1] = 0;
                an[i] -= x-an[i+1];
            }
            total += exceed
        }
    }
    println!("{}", total);
}
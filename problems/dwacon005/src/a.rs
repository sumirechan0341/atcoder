use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [i32; n]
    };
    let nmean = an.iter().sum::<i32>();
    let mut min_index = 0;
    for i in 0..n {
        if (nmean - (n as i32)*an[i]).abs() < (nmean-(n as i32)*an[min_index]).abs() {
            min_index = i;
        }
    }
    println!("{}", min_index);
}
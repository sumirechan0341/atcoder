use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [usize; n],
        bn: [usize; n]
    };
    let mut special_count = 0;
    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if an[i] == bn[j] {
                count += 1;
                if i == j {
                    special_count += 1;
                }
            }
        }
    }
    println!("{}", special_count);
    println!("{}", count-special_count);
}
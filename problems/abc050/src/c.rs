use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n]
    };
    let mut count = vec![0; n];
    let mut total: i64 = 1;
    let p = 1000000007;
    for i in 0..n {
        if an[i]^n == 0 {
            println!("{}", 0);
            return;
        }
        count[an[i]] += 1;
        if count[0] > 1 || count[an[i]] > 2 {
            println!("{}", 0);
            return;
        }
        if count[an[i]] == 2 {
            total *= 2;
            total %= p;
        }
    }
    println!("{}", total);
}
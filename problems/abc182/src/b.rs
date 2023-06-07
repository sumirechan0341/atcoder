use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        an: [i32; n]
    };
    let mut max = 0;
    let mut ans = 0;
    for i in 2..1001 {
        let mut count = 0;
        for a in &an {
            if a % i == 0 {
                count += 1;
            }
        }
        if count > max {
            max = count;
            ans = i;
        }
    }
    println!("{}", ans);
}
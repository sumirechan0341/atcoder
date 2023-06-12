use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: i32
    };

    let mut ans = 0;
    for i in 24..n+1 {
        let mut count = 0;
        for j in 1..n+1 {
            if i % j == 0 {
                count += 1;
            }
        }
        if count == 8 && i % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
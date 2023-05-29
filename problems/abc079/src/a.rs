use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: Chars
    };
    let mut count = 1;
    let mut pre = n[0];
    for i in 1..4 {
        if pre == n[i] {
            count += 1;
        } else {
            count = 1;
            pre = n[i];
        }
        if count > 2 {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
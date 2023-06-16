use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        m: i32
    };
    if m < 100 {
        println!("{}", "00");
        return;
    } else if m <= 5000 {
        println!("{:02}", m / 100);
        return;
    } else if 6000 <= m && m <= 30000 {
        println!("{}", m / 1000 + 50);
        return;
    } else if 35000 <= m && m <= 70000 {
        println!("{}", (m / 1000 - 30) / 5 + 80);
        return;
    } else {
        println!("{}", "89");
        return;
    }
}
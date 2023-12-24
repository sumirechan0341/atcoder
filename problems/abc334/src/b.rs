use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i128,
        m: i128,
        l: i128,
        r: i128
    };
    let l = l - a;
    let r = r - a;
    if l < 0 && r < 0 {
        println!(
            "{}",
            l.abs() / m - r.abs() / m + if r.abs() % m == 0 { 1 } else { 0 }
        );
        return;
    } else if l > 0 && r > 0 {
        println!(
            "{}",
            r.abs() / m - l.abs() / m + if l.abs() % m == 0 { 1 } else { 0 }
        );
        return;
    } else {
        println!("{}", l.abs() / m + r.abs() / m + 1);
    }
}

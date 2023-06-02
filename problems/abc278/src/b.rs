use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        h: i32,
        m: i32
    };
    let a = h / 10;
    let b = h % 10;
    let c = m / 10;
    let _d = m % 10;

    if b > 5 {
        // aは1もしくは0
        println!("{} {}", a * 10 + 10, 0);
    } else if a == 2 && c >= 4 {
        println!("{} {}", (h + 1) % 24, 0);
    } else {
        println!("{} {}", h, m);
    }
}

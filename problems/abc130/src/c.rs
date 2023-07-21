use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        w: i32,
        h: i32,
        x: i32,
        y: i32
    };
    let tate1 = x * h;
    let tate2 = (w-x) * h;

    let yoko1 = y * w;
    let yoko2 = (h - y) * w;
    println!("{}", tate1.min(tate2).max(yoko1.min(yoko2)));
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        hh: i32,
        ww: i32,
        h: i32,
        w: i32
    };
    if hh > ww && h > w {
        println!("{}", "Vertical");
    } else if hh > ww && h < w {
        println!("{}", "Horizontal");
    } else if hh < ww && h > w {
        println!("{}", "Horizontal");
    } else if hh < ww && h < w {
        println!("{}", "Vertical");
    } else {
        println!("{}", "Same");
    }
}
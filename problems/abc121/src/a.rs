use proconio::input;

pub fn main() {
    input! {
        hh: i32,
        ww: i32,
        h: i32,
        w: i32
    };
    println!("{}", hh * ww - h * ww - w * hh + h * w);
}
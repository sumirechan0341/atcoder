use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        sx: f32,
        sy: f32,
        gx: f32,
        gy: f32
    };
    println!("{}", -sy*(sx-gx)/(sy+gy)+sx);
}
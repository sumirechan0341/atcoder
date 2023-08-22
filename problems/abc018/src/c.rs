use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        r: usize,
        c: usize,
        k: usize,
        sr: [Chars; r]
    };
    for i in 0..2*k-1 {
        let n = k as i32  - (k as i32 -i as i32-1).abs();
        for j in -n+1..=n-1 {
            // println!("{} {}", i, j);
            // 中心からこの座標を全部しらべてoならOK
        }
    }
}

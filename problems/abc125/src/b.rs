use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        vn: [i32; n],
        cn: [i32; n]
    };
    let mut total = 0;
    for i in 0..n {
        if vn[i] - cn[i] > 0 {
            total += vn[i] - cn[i];
        }
    }
    println!("{}", total);
}
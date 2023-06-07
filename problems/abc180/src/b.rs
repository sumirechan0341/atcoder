use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        n: usize,
        xn: [i64; n]
    };
    let mut m = 0;
    for x in &xn {
        m += x.abs();
    }
    let mut e = 0.0;
    for x in &xn {
        e += x.pow(2) as f64;
    }
    e = e.sqrt();
    let mut c = 0;
    for x in &xn {
        if c < x.abs() {
            c = x.abs();
        }
    }
    println!("{}", m);
    println!("{}", e);
    println!("{}", c);   
}
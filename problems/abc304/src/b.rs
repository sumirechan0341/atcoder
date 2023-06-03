use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        n: f64
    };
    if n <= 10.0_f64.powf(3.0) - 1.0 {
        println!("{}", n);
    } else if n <= 10.0_f64.powf(4.0) - 1.0 {
        let base = 10.0_f64.powf(1.0);
        println!("{}", (n / base).floor() * base);
    } else if n <= 10.0_f64.powf(5.0) - 1.0 {
        let base = 10.0_f64.powf(2.0);
        println!("{}", (n / base).floor() * base);
    } else if n <= 10.0_f64.powf(6.0) - 1.0 {
        let base = 10.0_f64.powf(3.0);
        println!("{}", (n / base).floor() * base);
    } else if n <= 10.0_f64.powf(7.0) - 1.0 {
        let base = 10.0_f64.powf(4.0);
        println!("{}", (n / base).floor() * base);
    } else if n <= 10.0_f64.powf(8.0) - 1.0 {
        let base = 10.0_f64.powf(5.0);
        println!("{}", (n / base).floor() * base);
    } else if n <= 10.0_f64.powf(9.0) - 1.0 {
        let base = 10.0_f64.powf(6.0);
        println!("{}", (n / base).floor() * base);
    }
}
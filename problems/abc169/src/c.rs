use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i128,
        b: String
    };
    if a >= 10_i128.pow(3) {
        let large_a = (a - a % 100)/100;
        let small_a = a % 100;
        let mut bc = b.clone();
        let ib = bc.remove(1);
        println!("{}", bc.parse::<i128>().unwrap() * large_a + (small_a as f64 * b.parse::<f64>().unwrap()).floor() as i128);

    } else {
        let fb = b.parse::<f64>().unwrap();
        println!("{}", (a as f64*fb).floor());
    }
    
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i128, 
        b: i128
    };
    let mut lower = 0i128;
    let mut upper = 10i128.pow(18);
    while upper-lower > 1 {
        let mid = (upper+lower)/2;
        
        if mid.saturating_mul(mid).saturating_mul(mid).saturating_mul(4).saturating_mul(b).saturating_mul(b) < a*a {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    println!("{}", ((b*lower) as f64+a as f64/((1+lower) as f64).sqrt()).min((b*upper) as f64+a as f64/((1+upper) as f64).sqrt()));
}
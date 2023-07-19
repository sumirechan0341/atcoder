use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64
    };
    let mut start = 0;
    let mut end = 1000000001;
    loop {
        if end - start <= 1 {
            println!("{}", start);
            return;
        }
        if calc_yen(a, b, ((start + end) / 2).to_string()) > x {
            end = (start + end) / 2;
        } else {
            start = (start + end) / 2;
        }
    };
}

fn calc_yen(a: i64, b: i64, n: String) -> i64 {
    return a * n.parse::<i64>().unwrap() + b * (n.len() as i64);
}
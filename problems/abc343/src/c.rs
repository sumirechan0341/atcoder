use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize
    };
    let mut dict = vec![];
    for i in 1..=1000000 {
        let s = (i * i * i).to_string();
        let sc = s.clone().chars().rev().collect::<String>();
        if s == sc {
            dict.push(i * i * i);
        }
    }
    println!("{}", dict[dict.partition_point(|x| x <= &n) - 1]);
}

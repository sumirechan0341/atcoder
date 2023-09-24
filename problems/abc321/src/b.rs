use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        x: i32,
        mut an: [i32; n-1]
    };
    for i in 0..=100 {
        let mut anc = an.clone();
        anc.push(i);
        anc.sort();
        if x <= anc[1..n-1].iter().sum::<i32>() {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
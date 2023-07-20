use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut bn1: [i32; n-1]
    };
    if n == 2 {
        println!("{}", 2 * bn1[0]);
        return;
    }
    let mut an = vec![];
    an.push(bn1[0]);
    for i in 0..n-2 {
        an.push(bn1[i].min(bn1[i+1]));
    }
    an.push(bn1[n-2]);
    println!("{}", an.iter().sum::<i32>());
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let a = an.into_iter().filter(|x| x != &0);
    let num = a.clone().collect::<Vec<_>>().len() as i32;
    println!("{}", (a.clone().sum::<i32>() + num - 1) / num);
}
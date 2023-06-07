use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    println!("{}", an.into_iter().filter_map(|x| if x > 10 { Some(x-10) } else { None }).collect::<Vec<_>>().iter().sum::<i32>());
}
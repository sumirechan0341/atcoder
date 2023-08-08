use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut sn: [i32; n]
    };
    let total = sn.iter().sum::<i32>();
    if total%10 != 0 {
        println!("{}", total);
        return;
    }
    sn.sort();
    let mut candidates = sn.into_iter().filter(|&x| x%10 != 0);
    if candidates.clone().count() == 0 {
        println!("{}", 0);
        return;
    }
    println!("{}", total - candidates.nth(0).unwrap());
}
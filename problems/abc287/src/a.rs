use proconio::input;

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    }
    if sn.iter().filter(|&s| s == &"For").count() > n / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
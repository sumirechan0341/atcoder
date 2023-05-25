use proconio::input;

pub fn main() {
    input! {
        an: [i32; 3]
    };
    println!("{}", if an.iter().sum::<i32>() > 21 { "bust" } else { "win" });
}
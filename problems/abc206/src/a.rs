use proconio::input;

pub fn main() {
    input! {
        n: f32
    };
    let price = (1.08 * n).floor() as i32;
    println!("{}", if price < 206 { "Yay!" } else if price == 206 { "so-so" } else { ":(" });
}
use proconio::input;

pub fn main() {
    input !{
        n: u32
    };
    println!("{}", if n % 2 == 0 { n } else { n * 2 })
}   
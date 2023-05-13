use proconio::input;

pub fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    for i in 0..n {
        println!("{}", sn[n-i-1]);
    }
}
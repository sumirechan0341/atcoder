use proconio::input;

pub fn main() {
    input! {
        n: usize,
        abn: [[i32; 2]; n]
    }
    for ab in abn {
        println!("{}", ab[0] + ab[1]);
    }
}
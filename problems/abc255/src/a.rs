use proconio::input;

pub fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[u32; 2]; 2]
    };
    println!("{}", a[r-1][c-1]);
}
use proconio::input;

pub fn main() {
    input! {
        mut pqr: [i32; 3],
    };
    pqr.sort();
    println!("{}", pqr[0] + pqr[1]);
}
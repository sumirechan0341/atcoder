use proconio::input;

pub fn main() {
    input! {
        mut an: [i32; 4]
    };
    an.sort();
    println!("{}", an[0]);
}
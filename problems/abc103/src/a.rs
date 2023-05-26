use proconio::input;

pub fn main() {
    input !{
        mut an: [i32; 3]
    };
    an.sort();
    println!("{}", an[2] - an[0]);
}
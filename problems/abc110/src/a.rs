use proconio::input;

pub fn main() {
    input! {
        mut abc: [i32; 3]
    };
    abc.sort();
    println!("{}", abc[2] * 10 + abc[1] + abc[0]);
}
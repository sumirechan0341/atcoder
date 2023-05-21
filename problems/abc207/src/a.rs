use proconio::input;

pub fn main() {
    input! {
        mut abc: [u32; 3],
    };
    abc.sort();
    println!("{}", abc[2] + abc[1]);

}
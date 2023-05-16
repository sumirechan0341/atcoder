use proconio::input;

pub fn main() {
    input !{
        y: usize
    };
    println!("{}", if y % 4 == 2 { y } else if y % 4 == 0 { y + 2 } else { y + y % 4});
}
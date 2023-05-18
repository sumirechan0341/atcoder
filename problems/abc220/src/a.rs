use proconio::input;

pub fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32
    };
    for i in 0..1000 {
        if a <= c * i && c * i <= b {
            println!("{}", c * i);
            return;
        } 
    }
    println!("{}", -1);
}
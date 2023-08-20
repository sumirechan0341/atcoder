use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        m: usize,
        dm: [i32; m]
    };
    let mut half = dm.iter().sum::<i32>()/2;
    for i in 0.. {
        if dm[i] > half {
            println!("{} {}", i+1, half+1);
            return;
        } else {
            half -= dm[i]
        }
    }
}
use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        w: usize
    };
    let s = "DiscoPresentsDiscoveryChannelProgrammingContest2016";
    let mut j = 0;
    let mut end = false;
    loop {
        for i in 0..w {
            if j*w+i >= 50 {
                print!("{}", s.chars().nth(j*w + i).unwrap());
                println!("{}", "");
                end = true;
                break;
            }
            print!("{}", s.chars().nth(j*w + i).unwrap());
        }
        if end {
            break;
        }
        println!("{}", "");
        j += 1;
    }
}

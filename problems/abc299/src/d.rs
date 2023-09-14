use proconio::{input, marker::Chars, input_interactive};

pub fn main() {
    input_interactive! {
        n: usize
    };
    // let mut s = vec![0; n];
    // s[n-1] = 1;

    let mut left = 1_usize;
    let mut right = n;
    while right-left > 1 {
        let med = (left+right)/2;
        println!("? {}", med);
        input_interactive! {
            x: i32
        };
        if x == 0 {
            // s[med-1] = 0;
            left = med;
        } else {
            // s[med-1] = 1;
            right = med;
        }
    }
    
    println!("! {}", left);
}
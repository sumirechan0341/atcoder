use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: Chars
    };
    let mut index = (x.len()-1) as i32;
    while index >= 0 {
        match x[index as usize] {
            'o' | 'k' | 'u' => {
                index -= 1;
                continue;
            },
            'h' => {
                if index == 0 {
                    println!("{}", "NO");
                    return;
                }
                if x[(index-1) as usize] == 'c' {
                    index -= 2;
                    continue;
                } else {
                    println!("{}", "NO");
                    return;
                }

            },
            _ => {
                println!("{}", "NO");
                return;
            }
        } 
    }
    println!("{}", "YES");
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        x: i32,
        y: i32,
        z: i32
    };
    if x >= 0 && y >= 0 {
        if x - y >= 0 {
            // ハンマー必要
            if z >= 0 {
                if z > y {
                    // ハンマー取得不能
                    println!("{}", -1);
                } else {
                    println!("{}", x);
                }
            } else {
                println!("{}", -z * 2 + x);
            }
        } else {
            // ハンマー不要
            println!("{}", x);
        }
    } else if x < 0 && y < 0 {
        if y - x >= 0 {
            // ハンマー必要
            if z <= 0 {
                if z < y {
                    // ハンマー取得不能
                    println!("{}", -1);
                } else {
                    println!("{}", -x);
                }
            } else {
                println!("{}", z * 2 - x);
            }
        } else {
            // ハンマー不要
            println!("{}", -x);
        }
    } else {
        println!("{}", x.abs());
    }
}
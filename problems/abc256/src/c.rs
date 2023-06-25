use std::f32::consts::E;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        h1: u32,
        h2: u32,
        h3: u32,
        w1: u32, 
        w2: u32,
        w3: u32
    };
    let mut count = 0;
    for a1 in 1..29 {
        for a2 in 1..29 {
            for a3 in 1..29 {
                if a1 + a2 + a3 != h1 {
                    continue;
                }
                for a4 in 1.. 29 {
                    for a5 in 1..29 {
                        for a6 in 1.. 29 {
                            if a4 + a5 + a6 != h2 {
                                continue;
                            }
                            for a7 in 1.. 29 {
                                if a1 + a4 + a7 != w1 {
                                    continue;
                                }
                                for a8 in 1..29 {
                                    if a2 + a5 + a8 != w2 {
                                        continue;
                                    }
                                    for a9 in 1..29 {
                                        if a7 + a8 + a9 != h3 {
                                            continue;
                                        }
                                        if a3 + a6 + a9 != w3 {
                                            continue;
                                        }
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}
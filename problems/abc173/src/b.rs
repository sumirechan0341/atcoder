use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        sn: [String; n]
    };
    let mut jadge = [0; 4];
    for s in sn {
        match &*s {
            "AC" => {
                jadge[0] += 1;
            },
            "WA" => {
                jadge[1] += 1;
            },
            "TLE" => {
                jadge[2] += 1;
            },
            "RE" => {
                jadge[3] += 1
            }
            _ => {

            }
        }
    }
    println!("AC x {}", jadge[0]);
    println!("WA x {}", jadge[1]);
    println!("TLE x {}", jadge[2]);
    println!("RE x {}", jadge[3]);
}
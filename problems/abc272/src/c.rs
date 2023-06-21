use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        mut an: [usize; n]
    };
    an.sort();
    an.reverse();
    let mut even = 0;
    let mut odd = 1;
    let mut even_status = 0;
    let mut odd_status = 0;

    let mut max = -1;
    for a in an {
        if a % 2 == 1 {
            if odd_status == 0 {
                odd = a;
                odd_status += 1;
            } else if odd_status == 1 {
                odd += a;
                if max < odd as i32 {
                    max = odd as i32;
                }
                odd_status += 1;
            } else {
                continue;
            }
        } else {
            if even_status == 0 {
                even = a;
                even_status += 1;
            } else if even_status == 1 {
                even += a;
                if max < even as i32 {
                    max = even as i32;
                }
                even_status += 1;
            } else {
                continue;
            }
        }
    }
    println!("{}", max);
}
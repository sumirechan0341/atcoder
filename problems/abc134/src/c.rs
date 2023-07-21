use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i32; n]
    };
    let mut max = 0;
    let mut max_index = 0;
    let mut next_max = 0;
    for i in 0..n {
        if an[i] > max {
            max = an[i];
            max_index = i;
        } else if an[i] > next_max {
            next_max = an[i];
        }
    }
    for i in 0..n {
        println!("{}", if i == max_index { next_max } else { max });
    }
}
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        d: i64
    };
    let mut dict = vec![];
    for y in 0..=1414214 {
        dict.push(y * y);
    }
    let mut min = std::i64::MAX;
    for x in 0..=1414214 {
        let remain = d - x * x;
        if remain <= 0 {
            if min > x * x - d {
                min = x * x - d
            }
        } else {
            let index = dict.partition_point(|&y| y <= remain);
            let mut min2 = std::i64::MAX;
            if index <= 1000000 {
                min2 = (x * x + dict[index] - d).abs();
            }
            if index >= 1 {
                min2 = min2.min((x * x + dict[index - 1] - d).abs());
            }
            if index < 1000000 {
                min2 = min2.min((x * x + dict[index + 1] - d).abs());
            }
            if min > min2 {
                min = min2
            }
        }
    }
    println!("{}", min);
}

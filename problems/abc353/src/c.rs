use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [usize; n]
    };
    let mut can = an.clone();
    can.sort();
    let mut over = 0;
    let m = 100000000;

    for i in 0..n - 1 {
        over += n - (i + 1) - (can[i + 1..].partition_point(|x| x + can[i] < m));
    }
    let mut ans = 0;
    for i in 0..n {
        ans += an[i] * (n - 1);
    }
    println!("{}", ans - over * m);
}

fn guchoku(n: usize, an: Vec<usize>) -> usize {
    let m = 100000000;
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if an[i] + an[j] >= m {
                ans += an[i] + an[j] - m;
            } else {
                ans += an[i] + an[j];
            }
        }
    }
    ans
}

fn algo(n: usize, an: Vec<usize>) -> usize {
    let mut can = an.clone();
    can.sort();
    let mut over = 0;
    let m = 100000000;

    for i in 0..n - 1 {
        over += n - (i + 1) - (can[i + 1..].partition_point(|x| x + an[i] < m));
    }
    let mut ans = 0;
    for i in 0..n {
        ans += an[i] * (n - 1);
    }
    ans - over * m
}

// #[cfg(test)]
// // property based test
// mod tests {
//     use super::*;
//     use proptest::prelude::*;

//     proptest! {
//         #[test]
//         fn compare(an in (1..=10usize).prop_flat_map(|n| prop::collection::vec(1..100000000usize, n))) {
//             assert_eq!(guchoku(an.len(), an.clone()), algo(an.len(), an.clone()));
//         }
//     }
// }

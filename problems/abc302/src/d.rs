use proconio::input;

pub fn main() {
    input !{
        n: usize,
        m: usize,
        d: i64,
        mut an: [i64; n],
        mut bm: [i64; m]
    };
    an.sort();
    bm.sort();
    let mut cursor = 0;
    let mut ans = -1;

    let mut i = 0;
    while i < n {
        while cursor < m-1 && an[i] > bm[cursor+1] && an[i] - bm[cursor+1] > d {
            cursor += 1;
        }
        while i < n-1 && an[i+1] < bm[cursor] && bm[cursor] - an[i+1] > d {
            i += 1;
        }
        if an[i] + bm[cursor] > ans && (an[i] - bm[cursor]).abs() <= d {
            ans = an[i] + bm[cursor];
        }
        // 貪欲に先に延ばす
        while cursor < m-1 && (an[i] - bm[cursor+1]).abs() <= d {
            cursor += 1;
        }
        if an[i] + bm[cursor] > ans && (an[i] - bm[cursor]).abs() <= d {
            ans = an[i] + bm[cursor];
        }
        i += 1;
    }
    println!("{}", ans)
}

// fn main_test(d: i64, ant: Vec<i64>, bmt: Vec<i64>) -> i64 {
//     let n = ant.len();
//     let m = bmt.len();
//     let mut an = ant.clone();
//     let mut bm = bmt.clone();
//     an.sort();
//     bm.sort();
//     let mut cursor = 0;
//     let mut ans = -1;
//     println!("{:?}", an);
//     println!("{:?}", bm);
//     let mut i = 0;
//     while i < n {
//         while cursor < m-1 && an[i] > bm[cursor+1] && an[i] - bm[cursor+1] > d {
//             cursor += 1;
//         }
//         while i < n-1 && an[i+1] < bm[cursor] && bm[cursor] - an[i+1] > d {
//             i += 1;
//         }
//         if an[i] + bm[cursor] > ans && (an[i] - bm[cursor]).abs() <= d {
//             ans = an[i] + bm[cursor];
//         }
//         // 貪欲に先に延ばす
//         while cursor < m-1 && (an[i] - bm[cursor+1]).abs() <= d {
//             cursor += 1;
//         }
//         if an[i] + bm[cursor] > ans && (an[i] - bm[cursor]).abs() <= d {
//             ans = an[i] + bm[cursor];
//         }
//         i += 1;
//     }
//     return ans;
// }

// fn f(d: i64, an: Vec<i64>, bm: Vec<i64>) -> i64 {
//     let mut ans = -1;
//     for i in 0..an.len() {
//         for j in 0..bm.len() {
//              if (an[i]-bm[j]).abs() <= d && ans < an[i] + bm[j] {
//                 ans = an[i] + bm[j];
//              }
//         }
//     }
//     return ans;
// }
// use proptest::prelude::*;
// use proptest::collection::vec;
// use proptest::bool;
// proptest! {
//     #[test]
//     fn test(d in 0..5i64, an in vec(1..5i64, 5), bm in vec(1..5i64, 5)) {
//         prop_assert_eq!(main_test(d, an.clone(), bm.clone()), f(d, an, bm));
//     }
// }
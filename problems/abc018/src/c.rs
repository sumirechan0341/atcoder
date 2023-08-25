use proconio::{input, marker::Chars};
type VS = Vec<String>;
 
pub fn main() {
    input!{
        r: usize,
        c: usize,
        k: usize,
        sr: [Chars; r]
    };
    let mut row_seg = vec![vec![true; 1024]; r];
    let mut row_digit_num = 1;
    let mut c_cp = c;
    while c_cp != 0 {
        row_digit_num = row_digit_num << 1;
        c_cp = c_cp >> 1
    }
    for i in 0..r {
        for j in 0..row_digit_num {
            if j >= c {
                break;
            }
            row_seg[i][j+row_digit_num-1] = sr[i][j] == 'o';
        }
    }
    for i in 0..r {
        for j in (0..row_digit_num-1).rev() {
            row_seg[i][j] = row_seg[i][2*j+1] && row_seg[i][2*j+2];
        }
    }
    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            let mut ok = true;
            for l in 0..2*k-1 {
                let middle = (2*k-1)/2;
                let n = k-1 - (middle as i32 - l as i32).abs() as usize;
                if i+1 < k || j+1 < k || i+k-1 >= r || j+k-1 >= c {
                    ok = false;
                    break;
                }
                if !sat(&row_seg[i+l-middle], j-n, j+n+1, row_digit_num) {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);

    // let mut ans2 = 0;
    // for i in k-1..r-k+1 {
    //     for j in k-1..c-k+1 {
    //         let mut ok = true;
    //         for x in 0..r {
    //             for y in 0..c {
    //                 if (i as i32-x as i32).abs() + (j as i32-y as i32).abs() <= k as i32-1 {                        
    //                     if sr[x][y] == 'x' {
    //                         ok = false;
    //                         break;
    //                     }
    //                 } 
    //                 if !ok {
    //                     break;
    //                 }
    //             }
    //         }
    //         if ok {
    //             ans2 += 1;
    //         }
    //     }
    // }
    // println!("{}", ans2);
    
}
 
fn sat(seg: &Vec<bool>, l: usize, r: usize, n: usize) -> bool {
    let mut ll = l + (n-1);
    let mut rr = r + (n-1);
    let mut y = true;
    while ll < rr {
        if ll & 1 == 0{
            y = y && seg[ll];
        }
        if rr & 1 == 0{
            y = y && seg[rr-1];
        }
        ll = ll/2;
        rr = (rr-1)/2;
    }
    return y;
}
// セグメント木参考
// セグ木参考
// fn de(r: usize, c: usize, k: usize, sr: Vec<Vec<char>>) -> (i32, i32) {
//     let mut row_seg = vec![vec![true; 1024]; r];
//     let mut row_digit_num = 1;
//     let mut c_cp = c;
//     while c_cp != 0 {
//         row_digit_num = row_digit_num << 1;
//         c_cp = c_cp >> 1
//     }
//     for i in 0..r {
//         for j in 0..row_digit_num {
//             if j >= c {
//                 break;
//             }
//             row_seg[i][j+row_digit_num-1] = sr[i][j] == 'o';
//         }
//     }
//     for i in 0..r {
//         for j in (0..row_digit_num-1).rev() {
//             row_seg[i][j] = row_seg[i][2*j+1] && row_seg[i][2*j+2];
//         }
//     }
 
//     let mut ans = 0;
//     for i in 0..r {
//         for j in 0..c {
//             let mut ok = true;
//             for l in 0..2*k-1 {
//                 let middle = (2*k-1)/2;
//                 let n = k-1 - (middle as i32 - l as i32).abs() as usize;
//                 if i+1 < k || j+1 < k || i+k-1 >= r || j+k-1 >= c {
//                     ok = false;
//                     break;
//                 }
//                 if !sat(&row_seg[i+l-middle], j-n as usize, j+n+1, 0, 0, row_digit_num) {
//                     ok = false;
//                     break;
//                 }
//             }
//             if ok {
//                 ans += 1;
//             }
//         }
//     }

//     let mut ans2 = 0;
//     for i in k-1..r-k+1 {
//         for j in k-1..c-k+1 {
//             let mut ok = true;
//             for x in 0..r {
//                 for y in 0..c {
//                     if (i as i32-x as i32).abs() + (j as i32-y as i32).abs() <= k as i32-1 {                        
//                         if sr[x][y] == 'x' {
//                             ok = false;
//                             break;
//                         }
//                     } 
//                     if !ok {
//                         break;
//                     }
//                 }
//             }
//             if ok {
//                 ans2 += 1;
//             }
//         }
//     }
//     return (ans ,ans2);
// }
// proptest参考
// use proptest::prelude::*;
// use proptest::collection::vec;
// use proptest::bool;
// proptest! {
//     #[test]
//     fn main_test(k in 3usize..5, s in vec(vec(prop::bool::weighted(0.95), 10).prop_map(|x| x.iter().map(|&y| if y {'o'} else {'x'}).collect::<Vec<char>>()), 5)) {
//         let (l, r) = de(5, 10, k, s);
//         prop_assert_eq!(l, r);
//     }

        
// }
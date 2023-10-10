use std::collections::{HashMap, BTreeMap, HashSet};
use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mut n: usize,
        mut scn: [(i64, i64); n]
    };
    scn.sort();
    let mut map = HashMap::<i64, i64>::new();
    for i in 0..n {
        map.insert(scn[i].0, scn[i].1);
    }
    for i in 0..n {
        let mut digit = 0;
        while let Some(now) = map.get_mut(&(scn[i].0 << digit)) {
            let local_now = now.clone();
            if local_now == 0 {
                break;
            }          
            map.insert(scn[i].0 << digit, local_now%2);
            map.entry(scn[i].0 << (digit+1)).and_modify(|x| *x+=local_now/2).or_insert(local_now/2);
            digit += 1;
        }
    }
    let mut ans = 0;
    for (s, c) in map {
        ans += c;
    }
    println!("{}", ans);
    // prop_testで落ちるテストケース探した
    // let s = vec![
    //     (
    //         1,
    //         4,
    //     ),
    //     (
    //         4,
    //         2,
    //     ),
    //     (
    //         8,
    //         1,
    //     ),
    //     (
    //         2,
    //         6,
    //     ),
    // ];

    // println!("{}", main2(s.len(), s.clone()));
    // println!("{}", main3(s.len(), s.clone()));
}

// fn main2(n: usize, sc: Vec<(i64, i64)>) -> i64 {
//     let mut mp: BTreeMap<_, _> = sc.iter().cloned().collect();
//     let mut res = 0;

//     while let Some((s, c)) = mp.pop_first() {
// 	if c >= 2 {
// 	    *mp.entry(s * 2).or_default() += c / 2;
// 	}
// 	if c % 2 == 1 {
// 	    res += 1;
// 	}
//     }
//     return res;
// }

// fn main3(n: usize, sc: Vec<(i64, i64)>) -> i64 {
//     let mut scn = sc.clone();
//     scn.sort();
//     let mut map = HashMap::<i64, i64>::new();
//     for i in 0..n {
//         map.insert(scn[i].0, scn[i].1);
//     }
//     for i in 0..n {
//         let mut digit = 0;
//         while let Some(now) = map.clone().get_mut(&(scn[i].0 << digit)) {
//             if *now == 0 {
//                 break;
//             }          
//             map.insert(scn[i].0 << digit, *now%2);
//             map.entry(scn[i].0 << (digit+1)).and_modify(|x| *x+=*now/2).or_insert(*now/2);
//             digit += 1;
//         }
//     }
//     let mut ans = 0;
//     for (s, c) in map {
//         ans += c;
//     }
//     return ans;
// }

// use proptest::prelude::*;
// use proptest::collection::vec;
// use proptest::bool;
// proptest! {
//     #[test]
//     fn test(sc in vec((1..=10i64, 1..=10i64), 5)) {
//         let mut set = HashSet::<i64>::new();
//         let mut s = vec![];
//         for i in 0..sc.len() {
//             if set.contains(&sc[i].0) {

//             } else {
//                 s.push(sc[i]);
//                 set.insert(sc[i].0);
//             }
//         }
//         // println!("1 {}", main2(s.len(), s.clone()));
//         // println!("2 {}", main3(s.len(), s.clone()));
//         prop_assert_eq!(main2(s.len(), s.clone()), main3(s.len(), s.clone()));
//     }
// }

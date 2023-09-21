use std::collections::HashMap;
use proconio::{input, marker::{Chars, Usize1}};
use ac_library::{LazySegtree, MapMonoid, Monoid};
// セグ木 lazyに(bool, i64)を乗せるパターン
pub fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        q: usize
    };
    let mut ans = vec![];
    // 遅延セグ木の準備
    struct M;
    impl Monoid for M {
        type S = i64;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(&a: &Self::S, &b: &Self::S) -> Self::S {
            a
        }
    }
    struct F;
    impl MapMonoid for F {
        type M = M;
        type F = (bool, i64);

        fn identity_map() -> Self::F {
            (false, 0)
        }
        fn mapping(&f: &Self::F, &x: &<M as Monoid>::S) -> <M as Monoid>::S {
            if f.0 {
                f.1
            } else {
                x
            }
        }
        fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
            if f.0 {
                (true, f.1)
            } else if g.0 {
                (true, g.1)
            } else {
                (false, 0)
            }
        }
    }


    let mut seg = LazySegtree::<F>::new(n);
    for i in 0..n {
        seg.set(i, an[i]);
    }
    for _q in 0..q {
        input! {
            query_id: i32
        }
        if query_id == 1 {
            input! {
                x: i64
            }
            seg.apply_range(0..n, (true, x));
        } else if query_id == 2 {
            input! {
                idx: Usize1,
                x: i64
            }
            let y = seg.get(idx);
            seg.set(idx, x+y);
        } else {
            input! {
                idx: Usize1
            }
            ans.push(seg.get(idx).to_string());
        }
    }
    println!("{}", ans.join("\n"));
}

// セグ木 lazyに(i64, i64)を乗せるパターン

// use std::collections::HashMap;
// use proconio::{input, marker::{Chars, Usize1}};
// use ac_library::{LazySegtree, MapMonoid, Monoid};
// pub fn main() {
//     input! {
//         n: usize,
//         mut an: [i64; n],
//         q: usize
//     };
//     let mut now = -1i64;
//     let mut diff = HashMap::<usize, i64>::new();
//     let mut ans = vec![];
//     // 遅延セグ木の準備
//     struct M;
//     impl Monoid for M {
//         type S = i64;
//         fn identity() -> Self::S {
//             0
//         }
//         fn binary_operation(&a: &Self::S, &b: &Self::S) -> Self::S {
//             a
//         }
//     }
//     struct F;
//     impl MapMonoid for F {
//         type M = M;
//         type F = (i64, i64);

//         fn identity_map() -> Self::F {
//             (1, 0)
//         }
//         fn mapping(&f: &Self::F, &x: &<M as Monoid>::S) -> <M as Monoid>::S {
//             f.0 * x + f.1
//         }
//         fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
//             (f.0 * g.0, f.0 * g.1 + f.1)
//         }
//     }


//     let mut seg = LazySegtree::<F>::new(n);
//     for i in 0..n {
//         seg.set(i, an[i]);
//     }
    
//     for _q in 0..q {
//         input! {
//             query_id: i32
//         }
//         if query_id == 1 {
//             input! {
//                 x: i64
//             }
//             seg.apply_range(0..n, (0, x));
//         } else if query_id == 2 {
//             input! {
//                 idx: Usize1,
//                 x: i64
//             }
//             let y = seg.get(idx);
//             seg.set(idx, y+x);
//         } else {
//             input! {
//                 idx: Usize1
//             }
//             ans.push(seg.get(idx).to_string());
//         }
//     }
//     println!("{}", ans.join("\n"));
// }


// セグ木使わないパターン
// 差分配列をMapで持つ
// use std::collections::HashMap;

// use proconio::{input, marker::{Chars, Usize1}};

// pub fn main() {
//     input! {
//         n: usize,
//         mut an: [i64; n],
//         q: usize
//     };
//     let mut now = -1i64;
//     let mut diff = HashMap::<usize, i64>::new();
//     let mut ans = vec![];
//     for _q in 0..q {
//         input! {
//             query_id: i32
//         }
//         if query_id == 1 {
//             input! {
//                 x: i64
//             }
//             now = x;
//             diff = HashMap::<usize, i64>::new();
//         } else if query_id == 2 {
//             input! {
//                 idx: Usize1,
//                 x: i64
//             }
//             diff.entry(idx).and_modify(|y| *y += x).or_insert(x);
//         } else {
//             input! {
//                 idx: Usize1
//             }
//             let x = diff.get(&idx).unwrap_or(&0);
//             ans.push(if now == -1 { (an[idx]+x).to_string() } else { (now + x).to_string() });
//         }
//     }
//     println!("{}", ans.join("\n"));
// }
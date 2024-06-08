use ac_library::math::{inv_mod, pow_mod};
use ac_library::ModInt998244353 as Mint;
use proconio::input;

pub fn main() {
    input! {
        n: usize,
    };
    let r = Mint::from(10).pow(n.to_string().len() as u64);
    let result = Mint::from(n) * (Mint::from(r).pow(n as u64) - 1) / (r - 1);
    println!("{}", solve1(n));
    println!("{}", solve2(n));
    println!("{}", result);
}

fn solve1(n: usize) -> usize {
    let r = Mint::from(10).pow(n.to_string().len() as u64);
    let result = Mint::from(n) * (Mint::from(r).pow(n as u64) - 1) / (r - 1);
    return result.val() as usize;
}

fn solve2(n: usize) -> usize {
    let r = 10_usize.pow(n.to_string().len() as u32) % 998244353;
    let bunbo = inv_mod((r - 1) as i64, 998244353);
    let bunshi = pow_mod(r as i64, n as i64, 998244353) + 998244352 % 998244353;

    return (((n % 998244353) * bunshi as usize % 998244353) * bunbo as usize) % 998244353;
}

// #[cfg(test)]

// // write property-based tests
// mod tests {
//     use super::*;
//     use proptest::prelude::*;

//     proptest! {
//         #![proptest_config(ProptestConfig {
//             cases: 1000000, // テストケースの実行回数を1000に設定
//             .. ProptestConfig::default()
//         })]
//         #[test]
//         fn test(n in 1..=1000000 as usize) {
//             assert_eq!(solve1(n), solve2(n));
//         }
//     }
// }

use proconio::{input, marker::Chars};
use num::Integer;
pub fn main() {
    input! {
        n: i128,
        a: i128,
        b: i128
    };
    let total = n*(n+1)/2;
    let ac = n/a;
    let suma = ac*(ac+1)/2*a;
    let bc = n/b;
    let sumb = bc*(bc+1)/2*b;
    let abc = n/a.lcm(&b);
    let sumab = abc*(abc+1)/2*a.lcm(&b);
    println!("{}", total - (suma+sumb-sumab));
}
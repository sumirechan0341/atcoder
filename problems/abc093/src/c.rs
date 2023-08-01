use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32
    };
    let mut ns  = vec![a, b, c];
    ns.sort();
    let d1 = ns[2] - ns[0];
    let d2 = ns[2] - ns[1];
    if d1 % 2 == d2 % 2 {
        println!("{}", (d1+d2+1)/2);
        return;
    } else {
        let odd = if d1 % 2 == 0 { ns[1] } else { ns[0] };
        let even = if d1 % 2 == 0 { ns[0] } else { ns[1] };
        println!("{}", (ns[2]-odd+1)/2 + 1 + (ns[2]-even)/2);
        return;
    }
}
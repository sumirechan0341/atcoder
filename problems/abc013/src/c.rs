use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: i64,
        h: i64,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64
    };
    if e*n < h {
        println!("{}", 0);
        return;
    }
    let mut min = std::i64::MAX;
    for i in 0..n {
        let mut local_cost = 0;
        let hh = i*b + h;
        local_cost += i*a;
        // 残りn-i日間ある
        // 体力はhh
        let shortage = (n-i)*e-hh;
        if shortage >= 0 {
            local_cost += ((shortage)/(d+e)+1)*c;
        }
        if min > local_cost {
            min = local_cost;
        }
    }
    println!("{}", min);
}
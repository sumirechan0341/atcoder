use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    let mut d = vec![0; 100];
    if n != 0 {
        let mut nn = n.abs();
        for i in 0.. {
            if nn & 1 == 1 {
                carry(&mut d, i, 1, n > 0)
            }
            nn = nn >> 1;
            if nn == 0 {
                break;
            }
        }
    } else {
        println!("{}", 0);
        return
    }
    d.reverse();
    println!("{}", d.split_at(d.iter().position(|&x| x == 1).unwrap()).1.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));

}

fn carry(d: &mut Vec<i32>, cur: usize, bit: i32, sign: bool) {
    if bit == 0 {
        return;
    }
    if d[cur] == 0 {
        d[cur] = 1;
        let res = if sign { 1 } else { 0 };
        if cur % 2 == res {
            d[cur+1] = 1;
        }
    } else {
        d[cur] = 0;
        let res = if sign { 0 } else { 1 };
        if cur % 2 == res {
            d[cur+1] = 1;
            d[cur+2] = 1;
        } 
    }
}
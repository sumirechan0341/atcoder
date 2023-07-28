use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: i64
    };
    let mut d = vec![0; 100];
    if n > 0 {
        let mut nn = n;
        for i in 0.. {
            if nn & 1 == 1 {
                if i % 2 == 0 {
                    carry(&mut d, i, 1);
                } else {
                    carry(&mut d, i, 1);
                    carry(&mut d, i+1, 1);
                }  
            }
            nn = nn >> 1;
            if nn == 0 {
                break;
            }
        }
    }

    d.reverse();
    println!("{}", d.split_at(d.iter().position(|&x| x == 1).unwrap()).1.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
    // 以下0originで読む
    // +の数の場合
    // 1001 = 9
    // 奇数番目の1は-になる 打ち消すにはひとつ上から借りる必要がある
    // 11001 = 2^4 - 2^3 + 2^0
    // 下から見ていけばよい
    //  1111 = 15
    //  0001
    //  0111
    //  1011
    // 10011

    // -9
    // -の場合
    // 1001 = 9

}

fn carry(d: &mut Vec<i32>, cur: usize, bit: i32) {
    if bit == 0 {
        return;
    }
    if d[cur] == 0 {
        d[cur] = 1;
        return;
    } else {
        d[cur] = 0;
        if cur % 2 == 0 {
            carry(d, cur+1, 1);
            carry(d, cur+2, 1);
        } else {
            carry(d, cur+1, 1);
        }
        
    }
    
}
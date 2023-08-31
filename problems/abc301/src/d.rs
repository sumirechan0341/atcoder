use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        mut s: Chars,
        n: i64
    };
    let mut nd = vec![];
    let mut nn = n;
    for i in 0.. {
        if nn == 0 {
            break;
        }
        nd.push(nn % 2);
        nn /= 2;
    }
    nd.reverse();
    let mut ans = 0;
    if nd.len() > s.len() {
        for i in (0..s.len()).rev() {
            if s[i] == '?' {
                s[i] = '1';
            }
            ans += 2i64.pow((s.len()-i) as u32-1)*s[i].to_digit(10).unwrap() as i64;
        }
        println!("{}", ans);
        return;
    }
    if nd.len() < s.len() {
        for i in 0..s.len()-nd.len() {
            if s[i] == '1' {
                println!("{}", -1);
                return;
            }
            s.remove(0);
        }
    }

    let mut ok = false;
    for i in 0..s.len() {
        if s[i] == '?' {
            if ok {
                s[i] = '1';
            } else {
                if nd[i] == 0 {
                    s[i] = '0';
                    continue;
                }
                // 1が割り当て可能か見たい
                // 自分より右側の確認
                // ?は0で例化しておく
                for j in i+1..s.len() {
                    if s[j] == '?' {
                        if nd[j] == 1 {
                            s[i] = '1';
                            break;
                        } else {
                            continue;
                        }
                    } else {
                        if s[j].to_digit(10).unwrap() as i64 > nd[j] {
                            s[i] = '0';
                            break;
                        }
                    }
                }
                if s[i] == '?' {
                    s[i] = '1';
                }
            }
        } else {
            if s[i].to_digit(10).unwrap() as i64 > nd[i] && !ok {
                println!("{}", -1);
                return;
            }
            if (s[i].to_digit(10).unwrap() as i64) < nd[i] {
                ok = true;
            }
        }
    }
    for i in (0..s.len()).rev() {
        ans += 2i64.pow((s.len()-i) as u32-1)*s[i].to_digit(10).unwrap() as i64;
    }
    if ans > n {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
    
}
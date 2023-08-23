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
    let mut r_cp = r;
    while r_cp != 0 {
        row_digit_num = row_digit_num << 1;
        r_cp = r_cp >> 1
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
                if !sat(&row_seg[i+l-middle], j-n as usize, j+n+1, 0, 0, row_digit_num) {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("ans {}", ans);

    let mut ans2 = 0;
    for i in k-1..r-k+1 {
        for j in k-1..c-k+1 {
            let mut ok = true;
            for x in 0..r {
                for y in 0..c {
                    if (i as i32-x as i32).abs() + (j as i32-y as i32).abs() <= k as i32-1 {                        
                        if sr[x][y] == 'x' {
                            ok = false;
                            break;
                        }
                    } 
                    if !ok {
                        break;
                    }
                }
            }
            if ok {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans2);
    
}
 
fn sat(seg: &Vec<bool>, l: usize, r: usize, now: usize, now_l: usize, now_r: usize) -> bool {
    if now_r <= l || r <= now_l {
        return true;
    }
    if l <= now_l && now_r <= r {
        return seg[now];
    }
    let l_res = sat(seg, l, r, now*2+1, now_l, (now_l+now_r)/2);
    let r_res = sat(seg, l, r, now*2+2, (now_l+now_r)/2, now_r);
    return l_res && r_res;
}

fn de(r: usize, c: usize, k: usize, sr: Vec<Vec<char>>) -> (i32, i32) {
    let mut row_seg = vec![vec![true; 1024]; r];
    let mut row_digit_num = 1;
    let mut r_cp = r;
    while r_cp != 0 {
        row_digit_num = row_digit_num << 1;
        r_cp = r_cp >> 1
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
                if !sat(&row_seg[i+l-middle], j-n as usize, j+n+1, 0, 0, row_digit_num) {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("ans {}", ans);

    let mut ans2 = 0;
    for i in k-1..r-k+1 {
        for j in k-1..c-k+1 {
            let mut ok = true;
            for x in 0..r {
                for y in 0..c {
                    if (i as i32-x as i32).abs() + (j as i32-y as i32).abs() <= k as i32-1 {                        
                        if sr[x][y] == 'x' {
                            ok = false;
                            break;
                        }
                    } 
                    if !ok {
                        break;
                    }
                }
            }
            if ok {
                ans2 += 1;
            }
        }
    }
    println!("{}", ans2);
    return (ans ,ans2);
}

use proptest::prelude::*;
use proptest::collection::vec;
use proptest::char::*;
proptest! {
    #[test]
    fn main_test(k in 3usize..5, s in "(o{10},){5}") {
        let mut sr = vec![];
        for line in s.rsplit(",") {
            if line.is_empty() {
                continue;
            }
            sr.push(line.chars().collect::<Vec<char>>());
        }
        // println!("{:?}", sr);
        let (l, r) = de(5, 10, k, sr);
        prop_assert_eq!(l, r);
    }
    // fn main_test(r in 3..=50, c in 3..=50, k in 3..=50, sr in random_str_strategy(h, w)) {
    //     println!("{:?}", sr);
    // }

        
}

fn random_str_strategy(h: usize, w: usize) {

}

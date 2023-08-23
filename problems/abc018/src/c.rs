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
            let middle = (2*k-1)/2;
            for l in -middle..middle {
                // let n = k-1 - (middle as i32 - l as i32).abs() as usize;
                // if i+1 < k || j+1 < k || i+k-1 >= r || j+k-1 >= c {
                //     ok = false;
                //     break;
                // }
                // if !sat(&row_seg[i+l-middle], j-n as usize, j+n+1, 0, 0, row_digit_num) {
                //     ok = false;
                //     break;
                // }
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
    
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


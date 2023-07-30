use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        m: usize,
        sn: [Chars; n]
    };
    for i in 0..n-8 {
        for j in 0..m-8 {
            let mut ok = true;
            // 黒
            for k in 0..3 {
                for l in 0..3 {
                    if sn[i+k][j+l] == '.' || sn[i+8-k][j+8-l] == '.' {
                        ok = false;
                        break;
                    }
                }
            }
            // 白
            for k in 0..4 {
                if sn[i+3][j+k] == '#' || sn[i+k][j+3] == '#' || sn[i+5][j+5+k] == '#' || sn[i+5+k][j+5] == '#' {
                    ok = false;
                }
            }
            if ok {
                println!("{} {}", i+1, j+1);
            }
        }
    }
}
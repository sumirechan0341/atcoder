use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        ha: usize,
        wa: usize,
        aha: [Chars; ha],
        hb: usize,
        wb: usize,
        bhb: [Chars; hb],
        hx: usize,
        wx: usize,
        xhx: [Chars; hx],
    };
    let normalize_a = normalize(&aha, ha, wa);
    let normalize_b = normalize(&bhb, hb, wb);
    let normalize_x = normalize(&xhx, hx, wx);

    for i in 0..20 {
        for j in 0..20 {
            let mut map = vec![vec!['.'; 20]; 20];

            for k in 0..ha {
                for l in 0..wa {
                    map[k+hx][l+wx] = normalize_a[k][l];
                }
            }
            let mut b_ok = true;
            for k in 0..hb {
                for l in 0..wb {
                    if normalize_b[k][l] == '#' {
                        if i+k >= 20 || j+l >= 20 {
                            b_ok = false;
                            continue;
                        }
                        map[i+k][j+l] = normalize_b[k][l];
                    }
                }
            }
            if !b_ok {
                continue;
            }
            let normalize_map = normalize(&map, 20, 20);
            let mut ok = true;
            for k in 0..20 {
                for l in 0..20 {
                    if normalize_map[k][l] == '#' {
                        if k > hx-1 || l > wx-1 || normalize_x[k][l] != '#' {
                            ok = false;
                            continue;
                        }
                    } else {
                        if k > hx-1 || l > wx-1 || normalize_x[k][l] != '#' {
                            continue;
                        }
                        ok = false;
                    }
                }
            }
            if ok {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");

}

fn normalize(a: &Vec<Vec<char>>, h: usize, w: usize) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; w]; h];
    let mut min_w = w;
    let mut min_h = h;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                if i < min_h {
                    min_h = i;
                }
                if j < min_w {
                    min_w = j;
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                res[i-min_h][j-min_w] = '#';
            }
        }
    }
    return res;
}
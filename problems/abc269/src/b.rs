use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        mat: [Chars; 10]
    };
    let mut ystart = 0;
    let mut yend = 0;
    let mut xstart = 0;
    let mut xend = 0;
    
    for h in 0..10 {
        for w in 0..10 {
            if ystart == 0 && xstart == 0 && mat[h][w] == '#' {
                // 左上の頂点
                ystart = h + 1;
                xstart = w + 1;
            }
            if xstart != 0 && xend == 0 && mat[h][w] == '.' {
                xend = w;
            }
        }
        if xstart != 0 && xend == 0 {
            xend = 10;
        }
        if ystart != 0 && yend == 0 && mat[h][xstart-1] == '.' {
            yend = h;
        }
    }
    if yend == 0 {
        yend = 10;
    }
    println!("{} {}", ystart, yend);
    println!("{} {}", xstart, xend);
}
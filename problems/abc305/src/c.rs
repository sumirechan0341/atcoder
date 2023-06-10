use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input !{
        h: usize,
        w: usize,
        shw: [Chars; h]
    };
    let mut ws = vec![];
    let mut hs = vec![];
    for i in 0..h {
        let mut c = 0;
        for j in 0..w {
            if shw[i][j] == '#' {
                c += 1;
            }
        }
        ws.push((c, i+1));
    }
    for i in 0..w {
        let mut c = 0;
        for j in 0..h {
            if shw[j][i] == '#' {
                c += 1;
            }
        }
        hs.push((c, i+1));
    }

    ws.sort_by_key(|a| a.0);
    hs.sort_by_key(|a| a.0);

    let i = hs.iter().filter(|h| h.0 != 0).nth(0).unwrap().1;
    let j = ws.iter().filter(|w| w.0 != 0).nth(0).unwrap().1;
    println!("{} {}", j, i);
    
    
}
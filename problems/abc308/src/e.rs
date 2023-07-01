use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        n: usize,
        an: [usize; n],
        s: Chars
    };
    let mut xs = s.iter().enumerate().filter_map(|x| if x.1 == &'X' { Some(x.0) } else { None }).collect::<Vec<_>>();
    if let Some(last_x) = xs.iter().last() {
        let mut es = s.iter().enumerate().filter_map(|x| if x.1 == &'E' && x.0 < *last_x { Some(x.0) } else { None }).collect::<Vec<_>>();
        if let Some(last_m) = es.iter().last() {
            let mut ms = s.iter().enumerate().filter_map(|x| if x.1 == &'M' && x.0 < *last_m { Some(x.0) } else { None }).collect::<Vec<_>>();
            
            let mut msv = vec![0; 3];
            let mut esv = vec![0; 3];
            let mut xsv = vec![0; 3];
            let mut ans = 0;
            for i in ms {
                msv[an[i]] += 1;
            }
            for i in es {
                esv[an[i]] += 1;        
            }
            for i in xs {
                xsv[an[i]] += 1;        
            }
            println!("{:?}", msv);
            println!("{:?}", esv);
            println!("{:?}", xsv);
            for a in 0..3 {
                for b in 0..3 {
                    for c in 0..3 {
                        ans += mex(a, b, c)*msv[a]*esv[b]*xsv[c];
                    }
                }
            }
            println!("{}", ans);
            return;
        } else {
            println!("{}", 0);
            return;
        }
    } else {
        println!("{}", 0);
        return;
    }
    
}   
fn mex(a1: usize, a2: usize, a3: usize) -> usize {
    match (a1, a2, a3) {
        (0, 0, 0) => 1,
        (0, 0, 1) => 2,
        (0, 0, 2) => 1,
        (0, 1, 0) => 2,
        (0, 1, 1) => 2,
        (0, 1, 2) => 3,
        (0, 2, 0) => 1,
        (0, 2, 1) => 3,
        (0, 2, 2) => 1,
        (1, 0, 0) => 2,
        (1, 0, 1) => 2,
        (1, 0, 2) => 3,
        (1, 1, 0) => 2,
        (1, 1, 1) => 0,
        (1, 1, 2) => 0,
        (1, 2, 0) => 3,
        (1, 2, 1) => 0,
        (1, 2, 2) => 0,
        (2, 0, 0) => 1,
        (2, 0, 1) => 3,
        (2, 0, 2) => 1,
        (2, 1, 0) => 3,
        (2, 1, 1) => 0,
        (2, 1, 2) => 0,
        (2, 2, 0) => 1,
        (2, 2, 1) => 0,
        (2, 2, 2) => 0,
        _ => 0
    }
}
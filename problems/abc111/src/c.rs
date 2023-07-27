use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        vn: [usize; n]
    };
    let mut evene = vec![0; 100001];
    let mut odde = vec![0; 100001];
    for i in 0..n {
        if i % 2 == 0 {
            evene[vn[i]] += 1;
        } else {
            odde[vn[i]] += 1;
        }
    }
    let mut em = 0;
    let mut emi = 0;
    let mut emn = 0;
    let mut om = 0;
    let mut omi = 0;
    let mut omn = 0;
    for i in 0..100001 {
        if em < evene[i] {
            emn = em;
            em = evene[i];
            emi = i;
        } else if emn < evene[i] {
            emn = evene[i];
        }
        if om < odde[i] {
            omn = om;
            om = odde[i];
            omi = i;
        } else if omn < odde[i] {
            omn = odde[i];
        }
    }
    println!("{}", if emi != omi { n-em-om } else { n-(em+omn).max(om+emn) });
}
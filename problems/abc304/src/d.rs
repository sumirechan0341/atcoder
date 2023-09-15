use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        w: usize,
        h: usize,
        n: usize,
        mut pqn: [(usize, usize); n],
        a: usize,
        mut aa: [usize; a],
        b: usize,
        mut bb: [usize; b]
    };
    pqn.sort();
    let mut group = vec![];
    let mut cursor = 0;
    aa.push(w);
    for i in 0..a+1 {
        let mut local = vec![];
        while cursor < n && pqn[cursor].0 < aa[i] {
            local.push(pqn[cursor].1);
            cursor += 1;
        }
        group.push(local);
    }
    bb.sort();
    let mut min = n;
    let mut max = 0;
    for g in group {
        let mut count = vec![0; b+1];
        let mut lower_bound = 0;
        let mut low_num = 0;
        for i in g {
            let pos = bb.partition_point(|&x| x < i);
            if count[pos] == lower_bound {
                inc_num += 1;
            }
            if inc_num == b+1 {

            }
            count[pos] += 1;
            if max < count[pos] {
                max = count[pos];
            }
        }
    }
    println!("{} {}", min, max);
}

use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

pub fn main() {
    input! {
        n: usize,
        an: [i64; n],
        q: usize
    };
    let mut nodes = HashMap::new();
    if n == 1 {
        nodes.insert(an[0], (-1, i64::MAX / 2));
    } else {
        for i in 0..n {
            if i == 0 {
                nodes.insert(an[i], (-1, an[i + 1]));
            } else if i == n - 1 {
                nodes.insert(an[i], (an[i - 1], i64::MAX / 2));
            } else {
                nodes.insert(an[i], (an[i - 1], an[i + 1]));
            }
        }
    }

    for _ in 0..q {
        input! {
            id: usize
        }
        if id == 1 {
            input! {
                x: i64,
                y: i64
            }
            let mut next = i64::MAX / 2;
            let mut prev = -1;
            {
                let (_, n) = nodes.get_mut(&x).unwrap();
                next = *n;
                *n = y;
            }
            if next != i64::MAX / 2 {
                let (p, nn) = nodes.get_mut(&next).unwrap();
                prev = *p;
                *p = y;
            }
            nodes.insert(y, (x, next));
        } else {
            input! {
                x: i64
            }
            let (prev, next) = nodes[&x].clone();
            if prev != -1 {
                (nodes.get_mut(&prev).unwrap()).1 = next;
            }
            if next != i64::MAX / 2 {
                (nodes.get_mut(&next).unwrap()).0 = prev;
            }
            nodes.remove(&x);
        }
    }
    let mut anss = nodes.iter().collect::<Vec<_>>();
    anss.sort_by_key(|x| (x.1).0);
    let mut next = *anss[0].0;
    loop {
        println!("{}", next);
        if let Some(a) = nodes.get(&next) {
            next = a.1;
            if a.1 == i64::MAX / 2 {
                break;
            }
        }
    }
}

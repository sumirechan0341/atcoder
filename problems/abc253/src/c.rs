use std::collections::{BTreeSet, BTreeMap};
use proconio::{input, marker::Chars};
type VS = Vec<String>;
pub fn main() {
    input!{
        q: usize,
    };
    let mut ans = vec![];
    let mut multiset = BTreeMap::<i32, i32>::new();
    // multisetの例
    // 区間で消す操作は線形時間かかるため、そういうときはMapの方を使う
    for i in 0..q {
        input!{
            qt: i32,
        };
        match qt {
            1 => {
                input!{
                    x: i32,
                };
                multiset.entry(x).and_modify(|y| { *y += 1;}).or_insert(1);
            },
            2 => {
                input!{
                    x: i32,
                    c: i32
                };
                if let Some(y) = multiset.get_mut(&x) {
                    if &c >= y {
                        multiset.remove(&x);
                    } else {
                        *y -= c;
                    }
                }
            },
            _ => {
                ans.push(multiset.iter().last().unwrap().0 - multiset.iter().next().unwrap().0);
            }
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
use std::collections::HashSet;

use proconio::{input, marker::Chars};
type VS = Vec<String>;

pub fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32
    };
    // 操作は可換
    // 水だけ入れて、そのあとに砂糖を貪欲に入れる
    let mut max_rerational = (100*a, 0); // 分母, 分子
    let waters = dfs(0, 1, 100*a, 100*b, f);
    for water in waters {
        let sugar_limit = if water * e / 100 + water > f { f-water } else { water * e / 100 };
        let mut max = 0;
        for i in 0..=sugar_limit/d {
            let sugar_room = sugar_limit - i*d;
            if max < i*d + (sugar_room / c)*c {
                max = i*d + (sugar_room / c)*c;
            }
        }
        if max_rerational.0 * max > max_rerational.1*water {
            max_rerational = (water, max);
        }
    }
    println!("{} {}", max_rerational.0+max_rerational.1, max_rerational.1);
}

fn dfs(water: i32, prev_op: i32, a: i32, b: i32, f: i32) -> HashSet<i32> {
    let mut local_ans = HashSet::<i32>::new();
    if prev_op == 1 {
        // a, bどちらも選べる
        if water + a <= f {
            let new_water = water + a;
            local_ans.insert(new_water);
            for next in dfs(new_water, 1, a, b, f) {
                local_ans.insert(next);
            }
        }
        if water + b <= f {
            let new_water = water + b;
            local_ans.insert(new_water);
            for next in dfs(new_water, 2, a, b, f) {
                local_ans.insert(next);
            }
        }
    } else {
        // bしか選べない
        if water + b <= f {
            let new_water = water + b;
            local_ans.insert(new_water);
            for next in dfs(new_water, 2, a, b, f) {
                local_ans.insert(next);
            }
        }
    }
    return local_ans;
}